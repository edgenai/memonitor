use std::ops::{Deref, RangeBounds, RangeFull};
use std::sync::{RwLock, RwLockReadGuard};

#[cfg(feature = "vulkan")]
mod vulkan;

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

        #[cfg(feature = "vulkan")]
        if let Some((backend, devices)) = vulkan::Vulkan::init() {
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

pub fn init() {
    CONTEXT.write().unwrap().init();
}

pub fn list_backends() -> SliceGuard<'static, Backend, RangeFull> {
    let guard = CONTEXT.read().unwrap();
    SliceGuard {
        guard,
        range: ..,
        _phantom: Default::default(),
    }
}

pub fn list_all_devices() -> SliceGuard<'static, Device, RangeFull> {
    let guard = CONTEXT.read().unwrap();
    SliceGuard {
        guard,
        range: ..,
        _phantom: Default::default(),
    }
}

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

pub struct RefGuard<'s, T> {
    guard: RwLockReadGuard<'s, Context>,
    index: usize,
    _phantom: std::marker::PhantomData<T>,
}

impl<'s> Deref for RefGuard<'s, Backend> {
    type Target = Backend;

    fn deref(&self) -> &Self::Target {
        &self.guard.backends[self.index]
    }
}

impl<'s> Deref for RefGuard<'s, Device> {
    type Target = Device;

    fn deref(&self) -> &Self::Target {
        &self.guard.devices[self.index]
    }
}

pub trait BackendHandle: Send + Sync {
    fn name(&self) -> &str;
}

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
    pub fn id(&self) -> usize {
        self.id
    }

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

#[derive(Copy, Clone)]
pub enum DeviceKind {
    GPU(GPUKind),
    CPU,
    Other,
}

#[derive(Copy, Clone)]
pub enum GPUKind {
    Integrated,
    Discrete,
    Virtual,
}

pub struct MemoryStats {
    pub budget: usize,
    pub usage: usize,
}

pub trait DeviceHandle: Send + Sync {
    fn name(&self) -> &str;

    fn kind(&self) -> DeviceKind;

    fn backend_name(&self) -> &str;

    fn current_memory_stats(&self) -> MemoryStats;
}

pub struct Device {
    inner: Box<dyn DeviceHandle>,
    global_id: usize,
    local_id: usize,
    backend_id: usize,
}

impl Device {
    pub fn global_id(&self) -> usize {
        self.global_id
    }

    pub fn local_id(&self) -> usize {
        self.local_id
    }

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
