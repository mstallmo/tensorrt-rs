use crate::builder::Builder;
use crate::context::Context;
use crate::network::Network;
use crate::runtime::Runtime;
use std::convert::TryInto;
use std::ffi::{CStr, CString};
use std::os::raw::c_void;
use tensorrt_sys::{
    deserialize_cuda_engine, destroy_cuda_engine, engine_create_execution_context,
    get_binding_index, get_binding_name, get_nb_bindings,
};

pub struct Engine {
    pub(crate) internal_engine: *mut tensorrt_sys::Engine_t,
}

impl Engine {
    pub fn new(runtime: Runtime, buffer: Vec<u8>) -> Engine {
        let engine = unsafe {
            deserialize_cuda_engine(
                runtime.internal_runtime,
                buffer.as_ptr() as *const c_void,
                buffer.len() as u64,
            )
        };

        Engine {
            internal_engine: engine,
        }
    }

    pub fn from_builder(builder: &Builder, network: &Network) -> Engine {
        let engine = builder.build_cuda_engine(network);
        Engine {
            internal_engine: engine,
        }
    }

    pub fn get_nb_bindings(&self) -> i32 {
        unsafe { get_nb_bindings(self.internal_engine) }
    }

    pub fn get_binding_name(&self, binding_index: i32) -> Option<String> {
        if binding_index >= self.get_nb_bindings() {
            return None;
        }

        let binding_name = unsafe {
            let raw_binding_name =
                get_binding_name(self.internal_engine, binding_index.try_into().unwrap());
            CStr::from_ptr(raw_binding_name)
        };

        Some(binding_name.to_str().unwrap().to_string())
    }

    pub fn get_binding_index(&self, binding_name: &str) -> Option<i32> {
        let binding_index = unsafe {
            get_binding_index(
                self.internal_engine,
                CString::new(binding_name).unwrap().as_ptr(),
            )
        };

        if binding_index == -1 {
            return None;
        } else {
            return Some(binding_index);
        }
    }

    pub fn create_execution_context(&self) -> Context {
        let execution_context = unsafe { engine_create_execution_context(self.internal_engine) };
        Context {
            internal_context: execution_context,
            _engine: &self,
        }
    }
}

impl Drop for Engine {
    fn drop(&mut self) {
        unsafe { destroy_cuda_engine(self.internal_engine) };
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runtime::{Logger, Runtime};
    use std::fs::File;
    use std::io::prelude::*;

    fn setup_engine_test() -> Engine {
        let logger = Logger::new();
        let runtime = Runtime::new(&logger);

        let mut f = File::open("fix-me.engine").unwrap();
        let mut buffer = Vec::new();
        f.read_to_end(&mut buffer).unwrap();

        Engine::new(runtime, buffer)
    }

    #[test]
    fn get_nb_bindings() {
        let engine = setup_engine_test();

        assert_eq!(2, engine.get_nb_bindings());
    }

    #[test]
    fn get_engine_binding_name() {
        let engine = setup_engine_test();

        assert_eq!("data", engine.get_binding_name(0).unwrap());
    }

    #[test]
    fn get_invalid_engine_binding() {
        let engine = setup_engine_test();

        assert_eq!(None, engine.get_binding_name(2));
    }

    #[test]
    fn get_binding_index() {
        let engine = setup_engine_test();

        assert_eq!(Some(0), engine.get_binding_index("data"));
    }

    #[test]
    fn get_invalid_binding_index() {
        let engine = setup_engine_test();

        assert_eq!(None, engine.get_binding_index("not_valid"));
    }
}
