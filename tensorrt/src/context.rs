use crate::check_cuda;
use crate::profiler::{IProfiler, Profiler};
use anyhow::Error;
use cuda_runtime_sys::{cudaFree, cudaMalloc, cudaMemcpy, cudaMemcpyKind};
use ndarray;
use ndarray::Dimension;
use num_traits::Num;
use std::ffi::{CStr, CString};
use std::mem::size_of;
use std::os::raw::c_void;
use std::ptr;
use std::vec::Vec;
use tensorrt_sys::{
    context_get_debug_sync, context_get_name, context_set_debug_sync, context_set_name,
    context_set_profiler, destroy_excecution_context, execute, nvinfer1_IExecutionContext,
};

pub enum ExecuteInput<'a, D: Dimension> {
    Integer(&'a mut ndarray::Array<i32, D>),
    Float(&'a mut ndarray::Array<f32, D>),
}

struct DeviceBuffer {
    device_ptr: *mut c_void,
}

impl DeviceBuffer {
    pub fn new<T: Num, D: Dimension>(host_data: &ndarray::Array<T, D>) -> Result<Self, Error> {
        let mut device_ptr: *mut c_void = ptr::null_mut();
        check_cuda!(cudaMalloc(
            &mut device_ptr,
            host_data.len() * size_of::<T>()
        ));

        check_cuda!(cudaMemcpy(
            device_ptr,
            host_data.as_ptr() as *const c_void,
            host_data.len() * size_of::<T>(),
            cudaMemcpyKind::cudaMemcpyHostToDevice,
        ));

        Ok(DeviceBuffer { device_ptr })
    }

    pub fn new_uninit(size: usize) -> Result<Self, Error> {
        let mut device_ptr: *mut c_void = ptr::null_mut();
        check_cuda!(cudaMalloc(&mut device_ptr, size));
        Ok(DeviceBuffer { device_ptr })
    }

    pub fn as_mut_ptr(&self) -> *mut c_void {
        self.device_ptr
    }

    pub fn copy_to_host<T: Num, D: Dimension>(
        &self,
        host_data: &mut ndarray::Array<T, D>,
    ) -> Result<(), Error> {
        check_cuda!(cudaMemcpy(
            host_data.as_mut_ptr() as *mut c_void,
            self.device_ptr,
            host_data.len() * size_of::<T>(),
            cudaMemcpyKind::cudaMemcpyDeviceToHost,
        ));
        Ok(())
    }
}

impl Drop for DeviceBuffer {
    fn drop(&mut self) {
        if !self.device_ptr.is_null() {
            unsafe {
                cudaFree(self.device_ptr);
            }
        }
    }
}

pub struct Context {
    pub(crate) internal_context: *mut nvinfer1_IExecutionContext,
}

impl Context {
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

    pub fn set_profiler<P: IProfiler>(&self, profiler: &Profiler<P>) {
        unsafe { context_set_profiler(self.internal_context, profiler.internal_profiler) }
    }

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
        mut output_data: Vec<ExecuteInput<D2>>,
        batch_size: Option<i32>,
    ) -> Result<(), Error> {
        let mut buffers = Vec::<DeviceBuffer>::with_capacity(output_data.len() + 1);
        let dev_buffer = match input_data {
            ExecuteInput::Integer(val) => DeviceBuffer::new(val)?,
            ExecuteInput::Float(val) => DeviceBuffer::new(val)?,
        };
        buffers.push(dev_buffer);

        for output in &output_data {
            let device_buffer = match output {
                ExecuteInput::Integer(val) => {
                    DeviceBuffer::new_uninit(val.len() * size_of::<i32>())?
                }
                ExecuteInput::Float(val) => DeviceBuffer::new_uninit(val.len() * size_of::<f32>())?,
            };
            buffers.push(device_buffer);
        }

        let mut bindings = buffers
            .iter()
            .map(|elem| elem.as_mut_ptr())
            .collect::<Vec<*mut c_void>>();

        unsafe {
            execute(
                self.internal_context,
                bindings.as_mut_ptr(),
                batch_size.unwrap_or(1),
            );
        }

        for (idx, output) in buffers.iter().skip(1).enumerate() {
            let data = &mut output_data[idx];
            match data {
                ExecuteInput::Integer(val) => {
                    output.copy_to_host(val)?;
                }
                ExecuteInput::Float(val) => {
                    output.copy_to_host(val)?;
                }
            }
        }
        Ok(())
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe { destroy_excecution_context(self.internal_context) };
    }
}

unsafe impl Send for Context {}
unsafe impl Sync for Context {}

#[cfg(test)]
mod tests {
    use crate::builder::{Builder, NetworkBuildFlags};
    use crate::data_size::GB;
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
