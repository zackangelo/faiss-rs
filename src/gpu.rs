//! Contents for GPU support

use faiss_sys::*;
use error::Result;
use std::ptr;

/// Common interface for GPU resources used by Faiss.
pub trait GpuResources {
    /// Obtain a raw pointer to the native GPU resources object.
    fn inner_ptr(&self) -> *mut FaissGpuResources;

    /// Disable allocation of temporary memory; all temporary memory
    /// requests will call `cudaMalloc` / `cudaFree` at the point of use
    fn no_temp_memory(&mut self) -> Result<()>;

    /// Specify that we wish to use a certain fixed size of memory on
    /// all devices as temporary memory
    fn set_temp_memory(&mut self, size: usize) -> Result<()>;

    /// Specify that we wish to use a certain fraction of memory on
    /// all devices as temporary memory
    fn set_temp_memory_fraction(&mut self, fraction: f32) -> Result<()>;

    /// Set amount of pinned memory to allocate, for async GPU <-> CPU
    /// transfers
    fn set_pinned_memory(&mut self, size: usize) -> Result<()>;
}

/// Standard GPU resources descriptor.
/// 
/// # Examples
/// 
/// GPU resources are meant to be used via the [`into_gpu`] method
/// of an index.
/// 
/// ```
/// # fn run() -> Result<(), Box<::std::error::Error>> {
/// use faiss::{StandardGpuResources, MetricType};
/// use faiss::index::flat::FlatIndex;
///
/// let gpu = StandardGpuResources::new()?;
/// let index = FlatIndex::new(64, MetricType::L2)?;
/// let gpu_index = index.into_gpu(&gpu, 0)?;
/// # Ok(())
/// # }
/// # run().unwrap();
/// ```
///
/// Since GPU implementations are not thread-safe, attempting to
/// use the GPU resources from another thread will fail at
/// compile time.
/// 
/// ```compile_fail
/// use faiss::{GpuResources, StandardGpuResources, MetricType};
/// use faiss::index::flat::FlatIndex;
/// 
/// fn use_elsewhere<T: Sync>(_: &T) {}
/// 
/// # fn run() -> Result<(), Box<::std::error::Error>> {
/// let gpu = StandardGpuResources::new()?;
/// use_elsewhere(&gpu); // using GPU in another thread fails
/// # Ok(())
/// # }
/// # run().unwrap();
/// ```
/// 
/// Other than that, indexes can share the same GPU resources,
/// so long as neither of them cross any thread boundaries.
/// 
/// ```
/// use faiss::{GpuResources, StandardGpuResources, MetricType, index_factory};
/// 
/// # fn run() -> Result<(), Box<::std::error::Error>> {
/// let mut gpu = StandardGpuResources::new()?;
/// let index1 = index_factory(64, "Flat", MetricType::L2)?
///     .into_gpu(&gpu, 0)?;
/// let index2 = index_factory(32, "Flat", MetricType::L2)?
///     .into_gpu(&gpu, 0)?;
/// # Ok(())
/// # }
/// # run().unwrap();
/// ```
///
pub struct StandardGpuResources {
    inner: *mut FaissGpuResources,
}

// Deliberately _not_ Sync!
unsafe impl Send for StandardGpuResources {}

impl StandardGpuResources {
    /// Create a standard GPU resources object.
    pub fn new() -> Result<Self> {
        unsafe {
            let mut ptr = ptr::null_mut();
            faiss_try!(faiss_StandardGpuResources_new(&mut ptr));
            Ok(StandardGpuResources { inner: ptr })
        }
    }
}

impl GpuResources for StandardGpuResources {
    fn inner_ptr(&self) -> *mut FaissGpuResources {
        self.inner
    }

    fn no_temp_memory(&mut self) -> Result<()> {
        unsafe {
            faiss_try!(faiss_StandardGpuResources_noTempMemory(self.inner));
            Ok(())
        }
    }

    fn set_temp_memory(&mut self, size: usize) -> Result<()> {
        unsafe {
            faiss_try!(faiss_StandardGpuResources_setTempMemory(self.inner, size));
            Ok(())
        }
    }

    fn set_temp_memory_fraction(&mut self, fraction: f32) -> Result<()> {
        unsafe {
            faiss_try!(faiss_StandardGpuResources_setTempMemoryFraction(
                self.inner,
                fraction
            ));
            Ok(())
        }
    }

    fn set_pinned_memory(&mut self, size: usize) -> Result<()> {
        unsafe {
            faiss_try!(faiss_StandardGpuResources_setPinnedMemory(self.inner, size));
            Ok(())
        }
    }
}


impl<'g> GpuResources for &'g mut StandardGpuResources {
    fn inner_ptr(&self) -> *mut FaissGpuResources {
        self.inner
    }

    fn no_temp_memory(&mut self) -> Result<()> {
        (**self).no_temp_memory()
    }

    fn set_temp_memory(&mut self, size: usize) -> Result<()> {
        (**self).set_temp_memory(size)
    }

    fn set_temp_memory_fraction(&mut self, fraction: f32) -> Result<()> {
        (**self).set_temp_memory_fraction(fraction)
    }

    fn set_pinned_memory(&mut self, size: usize) -> Result<()> {
        (**self).set_pinned_memory(size)
    }
}

#[cfg(test)]
mod tests {
    use super::StandardGpuResources;

    #[test]
    fn smoke_detector() {
        StandardGpuResources::new().unwrap();
    }
}
