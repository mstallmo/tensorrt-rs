use std::marker::PhantomData;

use crate::engine::Engine;
use bitflags::_core::ffi::c_void;
use tensorrt_sys::{
    create_infer_runtime, create_logger, delete_logger, deserialize_cuda_engine,
    destroy_infer_runtime, nvinfer1_IRuntime, runtime_get_dla_core, runtime_get_nb_dla_cores,
    runtime_set_dla_core, set_logger_severity,
};

#[repr(C)]
pub enum LoggerSeverity {
    InternalError,
    Error,
    Warning,
    Info,
    Verbose,
}

pub struct Logger {
    pub(crate) internal_logger: *mut tensorrt_sys::Logger_t,
}

impl Logger {
    pub fn new() -> Logger {
        let logger = unsafe { create_logger(LoggerSeverity::Warning as i32) };
        Logger {
            internal_logger: logger,
        }
    }

    pub fn severity(self, severity: LoggerSeverity) -> Logger {
        unsafe {
            set_logger_severity(self.internal_logger, severity as i32);
        };
        self
    }
}

unsafe impl Send for Logger {}

impl Drop for Logger {
    fn drop(&mut self) {
        unsafe { delete_logger(self.internal_logger) };
    }
}

#[derive(Clone)]
pub struct Runtime<'a> {
    pub(crate) internal_runtime: *mut nvinfer1_IRuntime,
    pub(crate) logger: PhantomData<&'a Logger>,
}

impl<'a> Runtime<'a> {
    pub fn new(logger: &'a Logger) -> Self {
        let internal_runtime = unsafe { create_infer_runtime(logger.internal_logger) };
        let logger = PhantomData;
        Self {
            internal_runtime,
            logger,
        }
    }

    pub fn deserialize_cuda_engine(&self, buffer: Vec<u8>) -> Engine {
        let internal_engine = unsafe {
            deserialize_cuda_engine(
                self.internal_runtime,
                buffer.as_ptr() as *const c_void,
                buffer.len() as u64,
            )
        };

        Engine { internal_engine }
    }

    pub fn get_nb_dla_cores(&self) -> i32 {
        unsafe { runtime_get_nb_dla_cores(self.internal_runtime) }
    }

    pub fn get_dla_core(&self) -> i32 {
        unsafe { runtime_get_dla_core(self.internal_runtime) }
    }

    pub fn set_dla_core(&self, dla_core: i32) {
        unsafe { runtime_set_dla_core(self.internal_runtime, dla_core) }
    }
}

impl<'a> Drop for Runtime<'a> {
    fn drop(&mut self) {
        unsafe { destroy_infer_runtime(self.internal_runtime) };
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use lazy_static::lazy_static;
    use std::sync::Mutex;

    lazy_static! {
        static ref LOGGER: Mutex<Logger> = Mutex::new(Logger::new());
    }

    #[test]
    fn get_nb_dla_cores() {
        let logger = match LOGGER.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };

        let runtime = Runtime::new(&logger);

        assert_eq!(runtime.get_nb_dla_cores(), 0);
    }

    #[test]
    fn get_dla_core() {
        let logger = match LOGGER.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };

        let runtime = Runtime::new(&logger);

        assert_eq!(runtime.get_dla_core(), 0);
    }

    #[cfg(target_arch = "aarch64")]
    #[test]
    fn set_dla_core() {
        let logger = match LOGGER.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };

        let runtime = Runtime::new(&logger);
        runtime.set_dla_core(1);
        assert_eq!(runtime.get_dla_core(), 1);
    }
}
