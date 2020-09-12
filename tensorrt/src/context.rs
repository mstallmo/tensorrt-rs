use ndarray;
use ndarray::Dimension;
use std::ffi::{CStr, CString};
use std::mem::size_of;
use tensorrt_sys::{
    context_get_name, context_set_name, destroy_excecution_context, execute, Context_t,
};

pub struct Context<'a> {
    pub(crate) internal_context: *mut Context_t,
    pub(crate) _engine: &'a crate::engine::Engine,
}

impl<'a> Context<'a> {
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

    pub fn execute<D: Dimension>(
        &self,
        input_data: &ndarray::Array<f32, D>,
        input_binding_index: u32,
        output_binding_index: u32,
    ) -> ndarray::Array1<f32> {
        let mut output_array = ndarray::Array1::<f32>::zeros(10);
        unsafe {
            execute(
                self.internal_context,
                input_data.as_ptr(),
                input_data.len() * size_of::<f32>(),
                input_binding_index,
                output_array.as_mut_ptr(),
                output_array.len() * size_of::<f32>(),
                output_binding_index,
            );
        };
        output_array
    }
}

impl<'a> Drop for Context<'a> {
    fn drop(&mut self) {
        unsafe { destroy_excecution_context(self.internal_context) };
    }
}

#[cfg(test)]
mod tests {
    // use super::*;
    // use crate::engine::Engine;
    // use crate::runtime::{Logger, Runtime};
    // use std::fs::File;
    // use std::io::prelude::*;

    // fn setup_engine_test() -> Engine {
    //     let logger = Logger::new();
    //     let runtime = Runtime::new(&logger);
    //
    //     let mut f = File::open("../tensorrt-sys/resnet34-unet-Aug25-07-25-16-best.engine").unwrap();
    //     let mut buffer = Vec::new();
    //     f.read_to_end(&mut buffer).unwrap();
    //
    //     Engine::new(runtime, buffer)
    // }
    //
    // #[test]
    // fn set_context_name() {
    //     let engine = setup_engine_test();
    //     let mut context = engine.create_execution_context();
    //
    //     context.set_name("Mason");
    //     let name = context.get_name();
    //
    //     assert_eq!(name, "Mason".to_string());
    // }
}
