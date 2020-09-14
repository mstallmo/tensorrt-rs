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

    #[cfg(feature = "trt-515")]
    #[test]
    fn tensorrt_version() {
        let mut c_buf = Vec::<c_char>::with_capacity(6);
        unsafe { get_tensorrt_version(c_buf.as_mut_ptr()) };
        let c_str = unsafe { CStr::from_ptr(c_buf.as_ptr()) };
        assert_eq!("5.1.5", c_str.to_str().unwrap());
    }

    #[cfg(feature = "trt-713")]
    #[test]
    fn tensorrt_version() {
        let mut c_buf = Vec::<c_char>::with_capacity(6);
        unsafe { get_tensorrt_version(c_buf.as_mut_ptr()) };
        let c_str = unsafe { CStr::from_ptr(c_buf.as_ptr()) };
        assert_eq!("7.1.3", c_str.to_str().unwrap());
    }
}
