use std::ops::{Deref, Range, RangeBounds, RangeFull};
use std::sync::{Mutex, MutexGuard};

#[cfg(feature = "vulkan")]
mod vulkan;

static BACKENDS: Mutex<Vec<Box<dyn Backend>>> = Mutex::new(Vec::new());
static DEVICES: Mutex<Vec<Box<dyn Device>>> = Mutex::new(Vec::new());

pub fn init() {
    #[cfg(feature = "vulkan")]
    if let Some((backend, devices)) = vulkan::Vulkan::init() {
        register_backend(backend, devices)
    }
}

fn register_backend<B, D>(backend: B, devices: Vec<D>)
where
    B: Backend + 'static,
    D: Device + 'static,
{
    {
        let mut guard = BACKENDS.lock().unwrap();
        guard.push(Box::new(backend));
    }

    {
        let mut guard = DEVICES.lock().unwrap();
        for device in devices {
            guard.push(Box::new(device));
        }
    }
}

type OptionalSliceGuard<T, R> = Option<SliceGuard<'static, T, R>>;

pub fn list_backends() -> OptionalSliceGuard<Box<dyn Backend>, RangeFull> {
    if let Ok(guard) = BACKENDS.lock() {
        Some(SliceGuard { guard, range: .. })
    } else {
        None
    }
}

pub fn list_all_devices() -> OptionalSliceGuard<Box<dyn Device>, RangeFull> {
    if let Ok(guard) = DEVICES.lock() {
        Some(SliceGuard { guard, range: .. })
    } else {
        None
    }
}

pub struct SliceGuard<'s, T, R>
where
    R: RangeBounds<usize>,
{
    guard: MutexGuard<'s, Vec<T>>,
    range: R,
}

impl<'s, T, R> Deref for SliceGuard<'s, T, R>
where
    R: RangeBounds<usize>,
{
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        &self.guard[(
            self.range.start_bound().cloned(),
            self.range.end_bound().cloned(),
        )]
    }
}

pub struct RefGuard<'s, T> {
    guard: MutexGuard<'s, Vec<T>>,
    index: usize,
}

impl<'s, T> Deref for RefGuard<'s, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.guard[self.index]
    }
}

pub trait Backend: Send {
    fn name(&self) -> &str;

    fn device_count(&self) -> usize;

    fn list_devices(&self) -> OptionalSliceGuard<Box<dyn Device>, Range<usize>> {
        if let Ok(guard) = DEVICES.lock() {
            guard
                .iter()
                .position(|d| d.backend_name() == self.name())
                .map(|pos| SliceGuard {
                    guard,
                    range: pos..pos + self.device_count(),
                })
        } else {
            None
        }
    }
}

impl PartialEq for dyn Backend {
    fn eq(&self, other: &Self) -> bool {
        self.name() == other.name()
    }
}

impl Eq for dyn Backend {}

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

pub trait Device: Send {
    fn name(&self) -> &str;

    fn kind(&self) -> DeviceKind;

    fn backend_name(&self) -> &str;

    fn backend(&self) -> Option<RefGuard<Box<dyn Backend>>> {
        if let Ok(guard) = BACKENDS.lock() {
            guard
                .iter()
                .position(|b| b.name() == self.backend_name())
                .map(|pos| RefGuard { guard, index: pos })
        } else {
            None
        }
    }

    fn current_memory_stats(&self) -> MemoryStats;
}
