use crate::profiler::{IProfiler, ProfilerBinding};
use ndarray;
use ndarray::Dimension;
use std::ffi::{CStr, CString};
use std::mem::size_of;
use std::os::raw::c_void;
use std::vec::Vec;
use tensorrt_sys::{
    context_get_debug_sync, context_get_name, context_set_debug_sync, context_set_name,
    destroy_excecution_context, execute, nvinfer1_IExecutionContext, Profiler_t,
};

pub enum ExecuteInput<'a, D: Dimension> {
    Integer(&'a ndarray::Array<i32, D>),
    Float(&'a ndarray::Array<f32, D>),
}

pub struct Context<'a> {
    pub(crate) internal_context: *mut nvinfer1_IExecutionContext,
    pub(crate) _engine: &'a crate::engine::Engine,
}

impl<'a> Context<'a> {
    pub fn set_debug_sync(&self, sync: bool) {
        unsafe { context_set_debug_sync(self.internal_context, sync) }
    }

    pub fn get_debug_sync(&self) -> bool {
        unsafe { context_get_debug_sync(self.internal_context) }
    }

    pub fn set_name(&mut self, context_name: &str) {
        unsafe {
            context_set_name(
                self.internal_context,
                CString::new(context_name).unwrap().as_ptr(),
            )
        };
    }

    pub fn get_name(&self) -> String {
        let context_name = unsafe {
            let raw_context_name = context_get_name(self.internal_context);
            CStr::from_ptr(raw_context_name)
        };
        context_name.to_str().unwrap().to_string()
    }

    // pub fn set_profiler<T: IProfiler>(&self, profiler: &mut T) {
    //     let profiler_ptr =
    //         Box::into_raw(Box::new(ProfilerBinding::new(profiler))) as *mut Profiler_t;
    //     unsafe { context_set_profiler(self.internal_context, profiler_ptr) }
    // }
    //
    // pub fn get_profiler<T: IProfiler>(&self) -> &T {
    //     unsafe {
    //         let profiler_ptr =
    //             context_get_profiler(self.internal_context) as *mut ProfilerBinding<T>;
    //         &(*(*profiler_ptr).context)
    //     }
    // }

    pub fn execute<D1: Dimension, D2: Dimension>(
        &self,
        input_data: ExecuteInput<D1>,
        output_data: Vec<ExecuteInput<D2>>,
        num_bindings: i32,
    ) {
        let mut binding_data = match input_data {
            ExecuteInput::Integer(val) => vec![val.as_ptr() as *const c_void],
            ExecuteInput::Float(val) => vec![val.as_ptr() as *const c_void],
        };

        for data in &output_data {
            match data {
                ExecuteInput::Integer(val) => {
                    binding_data.push(val.as_ptr() as *const c_void);
                }
                ExecuteInput::Float(val) => {
                    binding_data.push(val.as_ptr() as *const c_void);
                }
            }
        }

        let mut data_sizes = match input_data {
            ExecuteInput::Integer(val) => vec![val.len() * size_of::<i32>()],
            ExecuteInput::Float(val) => vec![val.len() * size_of::<f32>()],
        };

        for data in &output_data {
            match data {
                ExecuteInput::Integer(val) => {
                    data_sizes.push(val.len() * size_of::<i32>());
                }
                ExecuteInput::Float(val) => {
                    data_sizes.push(val.len() * size_of::<f32>());
                }
            }
        }

        unsafe {
            execute(
                self.internal_context,
                binding_data.as_mut_ptr(),
                num_bindings,
                data_sizes.as_ptr(),
            );
        };
    }
}

impl<'a> Drop for Context<'a> {
    fn drop(&mut self) {
        unsafe { destroy_excecution_context(self.internal_context) };
    }
}

#[cfg(test)]
mod tests {
    use crate::builder::{Builder, NetworkBuildFlags};
    use crate::dims::DimsCHW;
    use crate::engine::Engine;
    use crate::profiler::RustProfiler;
    use crate::runtime::Logger;
    use crate::uff::{UffFile, UffInputOrder, UffParser};
    use lazy_static::lazy_static;
    use std::path::Path;
    use std::sync::Mutex;

    lazy_static! {
        static ref LOGGER: Mutex<Logger> = Mutex::new(Logger::new());
    }

    fn setup_engine_test_uff(logger: &Logger) -> Engine {
        let builder = Builder::new(&logger);
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
    fn set_debug_sync_true() {
        let logger = match LOGGER.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };
        let engine = setup_engine_test_uff(&logger);
        let context = engine.create_execution_context();

        context.set_debug_sync(true);
        assert_eq!(context.get_debug_sync(), true);
    }

    // Commenting this out until we can come up with a better solution to the `IProfiler`
    // interface binding.
    // #[test]
    // fn set_profiler() {
    //     let logger = match LOGGER.lock() {
    //         Ok(guard) => guard,
    //         Err(poisoned) => poisoned.into_inner(),
    //     };
    //     let engine = setup_engine_test_uff(&logger);
    //     let context = engine.create_execution_context();
    //
    //     let mut profiler = RustProfiler::new();
    //     context.set_profiler(&mut profiler);
    //
    //     let other_profiler = context.get_profiler::<RustProfiler>();
    //     assert_eq!(
    //         &profiler as *const RustProfiler,
    //         other_profiler as *const RustProfiler
    //     );
    // }
}
