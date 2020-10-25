use crate::context::Context;
use crate::dims::Dims;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use std::convert::TryInto;
use std::ffi::{CStr, CString};
use std::slice;
use tensorrt_sys::{
    destroy_host_memory, engine_binding_is_input, engine_create_execution_context,
    engine_create_execution_context_without_device_memory, engine_destroy,
    engine_get_binding_data_type, engine_get_binding_dimensions, engine_get_binding_index,
    engine_get_binding_name, engine_get_device_memory_size, engine_get_location,
    engine_get_max_batch_size, engine_get_nb_bindings, engine_get_nb_layers,
    engine_get_workspace_size, engine_is_refittable, engine_serialize, host_memory_get_data,
    host_memory_get_size, nvinfer1_ICudaEngine,
};

#[repr(C)]
#[derive(Debug, FromPrimitive, Eq, PartialEq)]
pub enum DataType {
    Float,
    Half,
    Int8,
    Int32,
}

#[repr(C)]
#[derive(Debug, FromPrimitive, Eq, PartialEq)]
pub enum TensorLocation {
    Host,
    Device,
}

#[derive(Debug)]
pub struct Engine {
    pub(crate) internal_engine: *mut nvinfer1_ICudaEngine,
}

impl Engine {
    pub fn get_nb_bindings(&self) -> i32 {
        let res = if !self.internal_engine.is_null() {
            unsafe { engine_get_nb_bindings(self.internal_engine) }
        } else {
            0
        };
        res
    }

    pub fn get_binding_name(&self, binding_index: i32) -> Option<String> {
        if binding_index >= self.get_nb_bindings() {
            return None;
        }

        let binding_name = unsafe {
            let raw_binding_name =
                engine_get_binding_name(self.internal_engine, binding_index.try_into().unwrap());
            CStr::from_ptr(raw_binding_name)
        };

        Some(binding_name.to_str().unwrap().to_string())
    }

    pub fn get_binding_index(&self, binding_name: &str) -> Option<i32> {
        let binding_index = unsafe {
            engine_get_binding_index(
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

    pub fn binding_is_input(&self, binding_index: i32) -> bool {
        unsafe { engine_binding_is_input(self.internal_engine, binding_index) }
    }

    pub fn get_binding_dimensions(&self, binding_index: i32) -> Dims {
        let raw_dims =
            unsafe { engine_get_binding_dimensions(self.internal_engine, binding_index) };

        Dims {
            internal_dims: raw_dims,
        }
    }

    pub fn get_binding_data_type(&self, binding_index: i32) -> DataType {
        let primitive =
            unsafe { engine_get_binding_data_type(self.internal_engine, binding_index) };
        FromPrimitive::from_i32(primitive).unwrap()
    }

    pub fn get_max_batch_size(&self) -> i32 {
        unsafe { engine_get_max_batch_size(self.internal_engine) }
    }

    pub fn get_nb_layers(&self) -> i32 {
        unsafe { engine_get_nb_layers(self.internal_engine) }
    }

    pub fn get_workspace_size(&self) -> usize {
        unsafe { engine_get_workspace_size(self.internal_engine) }
    }

    pub fn create_execution_context(&self) -> Context {
        let execution_context = unsafe { engine_create_execution_context(self.internal_engine) };
        Context {
            internal_context: execution_context,
            _engine: &self,
        }
    }

    pub fn create_execution_context_without_device_memory(&self) -> Context {
        let execution_context =
            unsafe { engine_create_execution_context_without_device_memory(self.internal_engine) };
        Context {
            internal_context: execution_context,
            _engine: &self,
        }
    }

    pub fn serialize(&self) -> HostMemory {
        let memory = unsafe { engine_serialize(self.internal_engine) };
        HostMemory { memory }
    }

    pub fn get_location(&self, binding_index: i32) -> TensorLocation {
        let primitive = unsafe { engine_get_location(self.internal_engine, binding_index) };
        FromPrimitive::from_i32(primitive).unwrap()
    }

    pub fn get_device_memory_size(&self) -> usize {
        unsafe { engine_get_device_memory_size(self.internal_engine) }
    }

    pub fn is_refittable(&self) -> bool {
        unsafe { engine_is_refittable(self.internal_engine) }
    }
}

unsafe impl Send for Engine {}

impl Drop for Engine {
    fn drop(&mut self) {
        if !self.internal_engine.is_null() {
            unsafe { engine_destroy(self.internal_engine) };
        }
    }
}

pub struct HostMemory {
    pub(crate) memory: *mut tensorrt_sys::nvinfer1_IHostMemory,
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
    use crate::builder::{Builder, NetworkBuildFlags};
    use crate::data_size::GB;
    use crate::dims::DimsCHW;
    use crate::runtime::{Logger, Runtime};
    use crate::uff::{UffFile, UffInputOrder, UffParser};
    use lazy_static::lazy_static;
    use std::fs::{remove_file, write, File};
    use std::io::prelude::*;
    use std::path::Path;
    use std::sync::Mutex;

    lazy_static! {
        static ref LOGGER: Mutex<Logger> = Mutex::new(Logger::new());
    }

    fn setup_engine_test_uff(logger: &Logger) -> Engine {
        let builder = Builder::new(&logger);
        builder.set_max_workspace_size(1 * GB);
        let network = builder.create_network_v2(NetworkBuildFlags::DEFAULT);

        let uff_parser = UffParser::new();
        let dim = DimsCHW::new(1, 28, 28);

        uff_parser
            .register_input("in", dim, UffInputOrder::Nchw)
            .unwrap();
        uff_parser.register_output("out").unwrap();
        let uff_file = UffFile::new(Path::new("../assets/lenet5.uff")).unwrap();
        uff_parser.parse(&uff_file, &network).unwrap();

        builder.build_cuda_engine(&network)
    }

    #[test]
    fn get_nb_bindings() {
        let logger = match LOGGER.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };
        let engine = setup_engine_test_uff(&logger);

        assert_eq!(2, engine.get_nb_bindings());
    }

    #[test]
    fn get_engine_binding_name() {
        let logger = match LOGGER.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };
        let engine = setup_engine_test_uff(&logger);

        assert_eq!("in", engine.get_binding_name(0).unwrap());
    }

    #[test]
    fn get_invalid_engine_binding() {
        let logger = match LOGGER.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };
        let engine = setup_engine_test_uff(&logger);

        assert_eq!(None, engine.get_binding_name(3));
    }

    #[test]
    fn binding_is_input() {
        let logger = match LOGGER.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };
        let engine = setup_engine_test_uff(&logger);

        assert_eq!(engine.binding_is_input(0), true);
    }

    #[test]
    fn get_binding_index() {
        let logger = match LOGGER.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };
        let engine = setup_engine_test_uff(&logger);

        assert_eq!(Some(0), engine.get_binding_index("in"));
    }

    #[test]
    fn get_binding_data_type() {
        let logger = match LOGGER.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };
        let engine = setup_engine_test_uff(&logger);

        assert_eq!(engine.get_binding_data_type(0), DataType::Float);
    }

    #[test]
    fn get_max_batch_size() {
        let logger = match LOGGER.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };
        let engine = setup_engine_test_uff(&logger);

        assert_eq!(engine.get_max_batch_size(), 1);
    }

    #[test]
    fn get_nb_layers() {
        let logger = match LOGGER.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };
        let engine = setup_engine_test_uff(&logger);

        assert_eq!(engine.get_nb_layers(), 7);
    }

    #[test]
    fn get_workspace_size() {
        let logger = match LOGGER.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };
        let engine = setup_engine_test_uff(&logger);

        assert_eq!(engine.get_workspace_size(), 0);
    }

    #[test]
    fn serialize() {
        let logger = match LOGGER.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };
        let uff_engine = setup_engine_test_uff(&logger);
        let seralized_path = Path::new("../lenet5.engine");
        write(seralized_path, uff_engine.serialize()).unwrap();

        assert!(seralized_path.exists());

        let logger = Logger::new();
        let runtime = Runtime::new(&logger);

        let mut f = File::open(seralized_path).unwrap();
        let mut buffer = Vec::new();
        f.read_to_end(&mut buffer).unwrap();

        let seralized_engine = runtime.deserialize_cuda_engine(buffer);

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

    #[test]
    fn get_location() {
        let logger = match LOGGER.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };
        let engine = setup_engine_test_uff(&logger);

        assert_eq!(engine.get_location(0), TensorLocation::Host);
    }

    #[test]
    fn get_device_memory_size() {
        let logger = match LOGGER.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };
        let engine = setup_engine_test_uff(&logger);

        assert_eq!(engine.get_device_memory_size(), 57856);
    }

    #[test]
    fn is_refittable() {
        let logger = match LOGGER.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };
        let engine = setup_engine_test_uff(&logger);

        assert_eq!(engine.is_refittable(), false);
    }
}
