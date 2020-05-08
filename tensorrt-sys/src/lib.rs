#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!("bindings.rs");

#[cfg(test)]
mod tests {
    use crate::{
        context_get_name, context_set_name, create_infer_runtime, create_logger, delete_logger,
        deserialize_cuda_engine, destroy_cuda_engine, destroy_excecution_context,
        destroy_infer_runtime, engine_create_execution_context, engine_serialize,
        get_binding_index, get_binding_name, get_tensorrt_version, host_memory_get_size,
        uffparser_create_uff_parser, uffparser_destroy_uff_parser, uffparser_register_input,
        uffparser_register_output,
    };
    use std::ffi::CStr;
    use std::ffi::CString;
    use std::fs::File;
    use std::io::prelude::*;
    use std::os::raw::{c_char, c_int, c_void};

    #[test]
    fn tensorrt_version() {
        let mut c_buf = Vec::<c_char>::with_capacity(6);
        unsafe { get_tensorrt_version(c_buf.as_mut_ptr()) };
        let c_str = unsafe { CStr::from_ptr(c_buf.as_ptr()) };
        assert_eq!("5.1.5", c_str.to_str().unwrap());
    }

    //    #[test]
    //    fn cuda_runtime() {
    //        let logger = unsafe { create_logger() };
    //        let runtime = unsafe { create_infer_runtime(logger) };
    //
    //        let mut f = File::open("resnet34-unet-Aug25-07-25-16-best.engine").unwrap();
    //        let mut buffer = Vec::new();
    //        f.read_to_end(&mut buffer).unwrap();
    //        let engine = unsafe {
    //            deserialize_cuda_engine(
    //                runtime,
    //                buffer.as_ptr() as *const c_void,
    //                buffer.len() as u64,
    //            )
    //        };
    //
    //        let bindingNameCStr = unsafe {
    //            let bindingName = get_binding_name(engine, 0);
    //            CStr::from_ptr(bindingName)
    //        };
    //        println!(
    //            "Binding name for index {} is {}",
    //            0,
    //            bindingNameCStr.to_str().unwrap()
    //        );
    //
    //        let execution_context = unsafe { engine_create_execution_context(engine) };
    //        unsafe { context_set_name(execution_context, CString::new("Mason").unwrap().as_ptr()) };
    //        let context_name_cstr = unsafe {
    //            let context_name = context_get_name(execution_context);
    //            CStr::from_ptr(context_name)
    //        };
    //        println!("Context name is {}", context_name_cstr.to_str().unwrap());
    //
    //        let input_binding =
    //            unsafe { get_binding_index(engine, CString::new("data").unwrap().as_ptr()) };
    //        println!("Binding index for data is {}", input_binding);
    //
    //        unsafe {
    //            destroy_excecution_context(execution_context);
    //            destroy_cuda_engine(engine);
    //            destroy_infer_runtime(runtime);
    //            delete_logger(logger);
    //        }
    //    }
    //
    //    #[test]
    //    fn host_memory() {
    //        let logger = unsafe { create_logger() };
    //        let runtime = unsafe { create_infer_runtime(logger) };
    //
    //        let mut f = File::open("resnet34-unet-Aug25-07-25-16-best.engine").unwrap();
    //        let mut buffer = Vec::new();
    //        f.read_to_end(&mut buffer).unwrap();
    //        let engine = unsafe {
    //            deserialize_cuda_engine(
    //                runtime,
    //                buffer.as_ptr() as *const c_void,
    //                buffer.len() as u64,
    //            )
    //        };
    //
    //        let host_memory = unsafe { engine_serialize(engine) };
    //        let memory_sise = unsafe { host_memory_get_size(host_memory) };
    //        println!("Host Memory Size of Engine: {}", memory_sise);
    //    }

    #[test]
    fn uff_parser() {
        let parser = unsafe { uffparser_create_uff_parser() };
        let mut d_vec = vec![3, 256, 256];
        let mut type_vec = vec![1, 0, 0];
        let dims = unsafe {
            crate::create_dims(
                3,
                d_vec.as_mut_ptr() as *mut c_int,
                type_vec.as_mut_ptr() as *mut c_int,
            )
        };
        let input_ret = unsafe {
            uffparser_register_input(parser, CString::new("input").unwrap().as_ptr(), dims)
        };
        assert_eq!(input_ret, true);

        let output_ret = unsafe {
            uffparser_register_output(parser, CString::new("sigmoid/Sigmoid").unwrap().as_ptr())
        };
        assert_eq!(output_ret, true);

        unsafe {
            uffparser_destroy_uff_parser(parser);
        }
    }
}
