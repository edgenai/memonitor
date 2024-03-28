//! **Memonitor** is a lightweight library that allows querying information from various CPU and GPU devices.
//! The main purpose is the ability to query memory related information, like how much local memory a device has and
//! how much is currently available to be allocated.
//!
//! This is achieved by dynamically loading, if present, various device APIs found in the system, and querying them
//! directly.
//!
//! # Example
//!
//! ```
//! use memonitor::{init, list_all_devices, list_backends};
//!
//! // Initialise global context
//! init();
//!
//! // Print every backend that has been found
//! for backend in list_backends().iter() {
//!     println!("Backend found: {}", backend.name());
//! }
//!
//! // Print every device found from every backend, as well as current memory statistics
//! for device in list_all_devices().iter() {
//!     let stats = device.current_memory_stats();
//!     println!(
//!         "Device found: {} ({}) - Memory stats: {} bytes used out of {}, {} are free",
//!         device.name(),
//!         device.kind(),
//!         stats.used,
//!         stats.total,
//!         stats.available
//!     );
//! }
//! ```
//!
//! # Features
//!
//! * `vulkan` - enables the Vulkan backend, enabled by default.

#![warn(missing_docs)]

use std::fmt::{Display, Formatter};
use std::ops::{Deref, RangeBounds, RangeFull};
use std::sync::{RwLock, RwLockReadGuard};

mod cpu;

/// The name of the always present CPU backend.
pub const CPU_NAME: &str = "Host";

#[cfg(feature = "vulkan")]
mod vulkan;

/// The name of the Vulkan backend.
#[cfg(feature = "vulkan")]
pub const VULKAN_NAME: &str = "Vulkan";

mod cuda;

/// The name of the Cuda backend.
#[cfg(feature = "vulkan")]
pub const CUDA_NAME: &str = "Cuda";

static CONTEXT: RwLock<Context> = RwLock::new(Context::default());

struct Context {
    backends: Vec<Backend>,
    devices: Vec<Device>,
}

impl Context {
    const fn default() -> Self {
        Context {
            backends: vec![],
            devices: vec![],
        }
    }

    fn init(&mut self) {
        if !self.backends.is_empty() {
            return;
        }

        let (system, cpus) = cpu::Host::init();
        self.register_backend(system, cpus);

        #[cfg(feature = "vulkan")]
        if let Some((backend, devices)) = vulkan::Vulkan::init() {
            self.register_backend(backend, devices)
        }

        #[cfg(feature = "cuda")]
        if let Some((backend, devices)) = cuda::Cuda::init() {
            self.register_backend(backend, devices)
        }
    }

    fn register_backend<B, D>(&mut self, backend: B, mut devices: Vec<D>)
    where
        B: BackendHandle + 'static,
        D: DeviceHandle + 'static,
    {
        let old_device_count = self.devices.len();
        let backend_id = self.backends.len();
        let mut new_device_ids = Vec::with_capacity(devices.len());

        for (idx, device) in devices.drain(..).enumerate() {
            let global_id = idx + old_device_count;
            let device = Device {
                inner: Box::new(device),
                global_id,
                local_id: idx,
                backend_id,
            };
            self.devices.push(device);
            new_device_ids.push(global_id);
        }

        let backend = Backend {
            inner: Box::new(backend),
            id: backend_id,
            device_ids: new_device_ids,
        };
        self.backends.push(backend);
    }
}

// TODO use OnceLock/OnceCell instead of forcing users to call this
/// Initialize the context. This **MUST** be called before any other functions.
///
/// This function is ideally called at the start of execution. If already called at least once, does nothing.
pub fn init() {
    CONTEXT.write().unwrap().init();
}

/// Returns a slice containing every [`Backend`] found.
///
/// The contents should always be identical after [`init`] has been called.
pub fn list_backends() -> SliceGuard<'static, Backend, RangeFull> {
    let guard = CONTEXT.read().unwrap();
    SliceGuard {
        guard,
        range: ..,
        _phantom: Default::default(),
    }
}

/// Returns a slice containing containing *every* [`Device`] found.
/// Depending on the [`Backend`]s present, there may be several representations for the same hardware device.
///
/// The contents should always be identical after [`init`] has been called.
pub fn list_all_devices() -> SliceGuard<'static, Device, RangeFull> {
    let guard = CONTEXT.read().unwrap();
    SliceGuard {
        guard,
        range: ..,
        _phantom: Default::default(),
    }
}

/// A type emulating a slice that holds a [`RwLockReadGuard`] of the inner context.
pub struct SliceGuard<'s, T, R>
where
    R: RangeBounds<usize>,
{
    guard: RwLockReadGuard<'s, Context>,
    range: R,
    _phantom: std::marker::PhantomData<T>,
}

impl<'s, R> Deref for SliceGuard<'s, Backend, R>
where
    R: RangeBounds<usize>,
{
    type Target = [Backend];

    fn deref(&self) -> &Self::Target {
        &self.guard.backends[(
            self.range.start_bound().cloned(),
            self.range.end_bound().cloned(),
        )]
    }
}

impl<'s, R> Deref for SliceGuard<'s, Device, R>
where
    R: RangeBounds<usize>,
{
    type Target = [Device];

    fn deref(&self) -> &Self::Target {
        &self.guard.devices[(
            self.range.start_bound().cloned(),
            self.range.end_bound().cloned(),
        )]
    }
}

/// Trait for internal backend handles.
pub trait BackendHandle: Send + Sync {
    /// Returns the name of this backend.
    fn name(&self) -> &str;
}

/// High-level abstraction over a backend.
pub struct Backend {
    inner: Box<dyn BackendHandle>,
    id: usize,
    device_ids: Vec<usize>,
}

impl PartialEq for Backend {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Backend {}

impl Backend {
    /// Return the *id* of this backend within the context.
    ///
    /// Used for indexing into slices returned by [`list_backends`].
    pub fn id(&self) -> usize {
        self.id
    }

    /// Return the *id*s of [Device]s owned by this backend.
    ///
    /// Used for indexing into slices returned by [`list_all_devices`].
    pub fn device_ids(&self) -> &[usize] {
        &self.device_ids
    }
}

impl Deref for Backend {
    type Target = Box<dyn BackendHandle>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

/// The hardware type of a [`Device`].
#[derive(Copy, Clone)]
pub enum DeviceKind {
    /// A Graphics Processing Unit a.k.a. a Graphics Card.
    GPU(GPUKind),
    /// A Central Processing Unit.
    CPU,
    /// Some other, unknown type.
    Other,
}

impl Display for DeviceKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DeviceKind::GPU(GPUKind::Integrated) => write!(f, "Integrated Graphics Card"),
            DeviceKind::GPU(GPUKind::Discrete) => write!(f, "Discrete Graphics Card"),
            DeviceKind::GPU(GPUKind::Virtual) => write!(f, "Virtual Graphics Card"),
            DeviceKind::CPU => write!(f, "CPU"),
            DeviceKind::Other => write!(f, "Other"),
        }
    }
}

/// The type of a Graphics Card.
#[derive(Copy, Clone)]
pub enum GPUKind {
    /// A Graphics Card physically integrated into the CPU (probably sharing the same memory).
    Integrated,
    /// A discrete Graphics Card, probably connected through PCIE.
    Discrete,
    /// A virtual Graphics Card.
    Virtual,
}

/// Memory information of a device at some point in time.
pub struct MemoryStats {
    /// The total local memory, in bytes. This value should always be the same for the same [`Device`].
    pub total: usize,
    /// The current amount of local memory that is available for new allocations, in bytes.
    pub available: usize,
    /// The current amount of local memory in use by the system, in bytes.
    pub used: usize,
}

/// Trait for internal device handles.
pub trait DeviceHandle: Send + Sync {
    /// Return the name of this device.
    fn name(&self) -> &str;

    /// Return the hardware type of this device.
    fn kind(&self) -> DeviceKind;

    /// Return the name of the [`Backend`] that owns this device handle.
    fn backend_name(&self) -> &str;

    /// Return the memory statistics of this device at this point in time.
    fn current_memory_stats(&self) -> MemoryStats;
}

/// High-level abstraction over a hardware device.
pub struct Device {
    inner: Box<dyn DeviceHandle>,
    global_id: usize,
    local_id: usize,
    backend_id: usize,
}

impl Device {
    /// Return the *id* of this device within the global context.
    ///
    /// Used for indexing into slices returned by [`list_all_devices`].
    pub fn global_id(&self) -> usize {
        self.global_id
    }

    /// Return the *id*/index of this device within its owning [`Backend`].
    ///
    /// This could be used, for example: to select this device in a CUDA context.
    pub fn local_id(&self) -> usize {
        self.local_id
    }

    /// Return the *id* of this device's owning [`Backend`].
    ///
    /// Used for indexing into slices returned by [`list_backends`].
    pub fn backend_id(&self) -> usize {
        self.backend_id
    }
}

impl Deref for Device {
    type Target = Box<dyn DeviceHandle>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
