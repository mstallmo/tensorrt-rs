use std::convert::TryInto;
use std::ffi::{CStr, CString};
use std::marker::PhantomData;
use std::os::raw::c_void;
use std::slice;

use crate::context::Context;
use crate::dims::Dims;
use crate::runtime::{Logger, Runtime};
use tensorrt_sys::{
    deserialize_cuda_engine, destroy_cuda_engine, destroy_host_memory,
    engine_create_execution_context, engine_serialize, get_binding_dimensions, get_binding_index,
    get_binding_name, get_nb_bindings, host_memory_get_data, host_memory_get_size,
};

#[derive(Debug)]
pub struct Engine<'a> {
    pub(crate) internal_engine: *mut tensorrt_sys::Engine_t,
    pub(crate) logger: PhantomData<&'a Logger>,
}

impl<'a> Engine<'a> {
    pub fn new(runtime: Runtime<'a>, buffer: Vec<u8>) -> Self {
        let internal_engine = unsafe {
            deserialize_cuda_engine(
                runtime.internal_runtime,
                buffer.as_ptr() as *const c_void,
                buffer.len() as u64,
            )
        };
        let logger = PhantomData;

        Self {
            internal_engine,
            logger,
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

        return if binding_index == -1 {
            None
        } else {
            Some(binding_index)
        };
    }

    pub fn get_binding_dimensions(&self, binding_index: i32) -> Dims {
        let raw_dims = unsafe { get_binding_dimensions(self.internal_engine, binding_index) };

        Dims {
            internal_dims: raw_dims,
        }
    }

    pub fn create_execution_context(&self) -> Context {
        let execution_context = unsafe { engine_create_execution_context(self.internal_engine) };
        Context {
            internal_context: execution_context,
            _engine: &self,
        }
    }

    pub fn serialize(&self) -> HostMemory {
        let memory = unsafe { engine_serialize(self.internal_engine) };
        HostMemory { memory }
    }
}

unsafe impl<'a> Send for Engine<'a> {}

impl<'a> Drop for Engine<'a> {
    fn drop(&mut self) {
        unsafe { destroy_cuda_engine(self.internal_engine) };
    }
}

pub struct HostMemory {
    pub(crate) memory: *mut tensorrt_sys::HostMemory_t,
}

impl HostMemory {
    pub fn data(&self) -> &[u8] {
        let ptr = unsafe { host_memory_get_data(self.memory) };
        let size = unsafe { host_memory_get_size(self.memory) };
        unsafe { slice::from_raw_parts(ptr as *const u8, size) }
    }
}

impl AsRef<[u8]> for HostMemory {
    fn as_ref(&self) -> &[u8] {
        self.data()
    }
}

impl Drop for HostMemory {
    fn drop(&mut self) {
        unsafe {
            destroy_host_memory(self.memory);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::builder::Builder;
    use crate::dims::DimsCHW;
    use crate::runtime::{Logger, Runtime};
    use crate::uff::{UffFile, UffInputOrder, UffParser};
    use lazy_static::lazy_static;
    use std::fs::{remove_file, write, File};
    use std::io::prelude::*;
    use std::path::Path;
    use std::sync::Mutex;

    lazy_static! {
        static ref ENGINE: Mutex<Engine> = Mutex::new(setup_engine_test_uff());
    }

    fn setup_engine_test_uff() -> Engine {
        let logger = Logger::new();
        let builder = Builder::new(&logger);

        let uff_parser = UffParser::new();
        let dim = DimsCHW::new(1, 28, 28);

        uff_parser
            .register_input("in", dim, UffInputOrder::Nchw)
            .unwrap();
        uff_parser.register_output("out").unwrap();
        let uff_file = UffFile::new(Path::new("../lenet5.uff")).unwrap();
        uff_parser.parse(&uff_file, builder.get_network()).unwrap();

        builder.build_cuda_engine()
    }

    #[test]
    fn get_nb_bindings() {
        let engine = ENGINE.lock().unwrap();

        assert_eq!(2, engine.get_nb_bindings());
    }

    #[test]
    fn get_engine_binding_name() {
        let engine = ENGINE.lock().unwrap();
        assert_eq!("in", engine.get_binding_name(0).unwrap());
    }

    #[test]
    fn get_invalid_engine_binding() {
        let engine = ENGINE.lock().unwrap();
        assert_eq!(None, engine.get_binding_name(3));
    }

    #[test]
    fn get_binding_index() {
        let engine = ENGINE.lock().unwrap();

        assert_eq!(Some(0), engine.get_binding_index("in"));
    }

    #[test]
    fn write_and_read_engine() {
        let uff_engine: &Engine = &*ENGINE.lock().unwrap();
        let seralized_path = Path::new("../lenet5.engine");
        write(seralized_path, uff_engine.serialize()).unwrap();

        assert!(seralized_path.exists());

        let logger = Logger::new();
        let runtime = Runtime::new(&logger);

        let mut f = File::open(seralized_path).unwrap();
        let mut buffer = Vec::new();
        f.read_to_end(&mut buffer).unwrap();

        let seralized_engine = Engine::new(runtime, buffer);

        assert_eq!(
            uff_engine.get_nb_bindings(),
            seralized_engine.get_nb_bindings()
        );

        for i in 0..uff_engine.get_nb_bindings() {
            assert_eq!(
                uff_engine.get_binding_name(i),
                seralized_engine.get_binding_name(i)
            );
            assert_eq!(
                uff_engine.get_binding_name(i),
                seralized_engine.get_binding_name(i)
            );
        }

        remove_file(seralized_path).unwrap();
    }
}
