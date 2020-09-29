use ndarray;
use ndarray::Dimension;
use std::ffi::{CStr, CString};
use std::mem::size_of;
use std::os::raw::c_void;
use std::vec::Vec;
use tensorrt_sys::{
    context_get_name, context_set_name, destroy_excecution_context, execute, Context_t,
};

pub enum ExecuteInput<'a, D: Dimension> {
    Integer(&'a ndarray::Array<i32, D>),
    Float(&'a ndarray::Array<f32, D>),
}

pub struct Context<'a, 'b> {
    pub(crate) internal_context: *mut Context_t,
    pub(crate) _engine: &'a crate::engine::Engine<'b>,
}

impl<'a, 'b> Context<'a, 'b> {
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

impl<'a, 'b> Drop for Context<'a, 'b> {
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
