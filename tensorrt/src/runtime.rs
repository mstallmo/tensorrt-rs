use std::marker::PhantomData;

use tensorrt_sys::{
    create_infer_runtime, create_logger, delete_logger, destroy_infer_runtime,
    runtime_get_nb_dla_cores, set_logger_severity,
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
    pub(crate) internal_runtime: *mut tensorrt_sys::Runtime_t,
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

    pub fn get_nb_dla_cores(&self) -> i32 {
        unsafe { runtime_get_nb_dla_cores(self.internal_runtime) }
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
}
