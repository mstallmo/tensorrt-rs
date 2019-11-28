use crate::network::Network;
use std::ffi::CString;
use std::io::{Error, ErrorKind};
use std::path::Path;
use tensorrt_sys::{
    uffparser_create_uff_parser, uffparser_destroy_uff_parser, uffparser_parse,
    uffparser_register_input, uffparser_register_output, Dims,
};

pub struct UffParser {
    internal_uffparser: *mut tensorrt_sys::UffParser_t,
}

impl UffParser {
    pub fn new() -> UffParser {
        let parser = unsafe { uffparser_create_uff_parser() };
        UffParser {
            internal_uffparser: parser,
        }
    }

    pub fn register_input(&self, input_name: &str) {
        let mut d_vec = vec![3, 256, 256];
        let mut type_vec = vec![1, 0, 0];
        let dims = Dims {
            nbDims: 3,
            d: d_vec.as_mut_ptr(),
            type_: type_vec.as_mut_ptr(),
        };
        unsafe {
            uffparser_register_input(
                self.internal_uffparser,
                CString::new(input_name).unwrap().as_ptr(),
                dims,
            )
        };
    }

    pub fn register_output(&self, output_name: &str) {
        unsafe {
            uffparser_register_output(
                self.internal_uffparser,
                CString::new(output_name).unwrap().as_ptr(),
            )
        };
    }

    pub fn parse(&self, file_path: &Path, network: &Network) -> Result<(), std::io::Error> {
        if !file_path.exists() {
            return Err(Error::new(ErrorKind::NotFound, "UFF file does not exist"));
        }

        unsafe {
            uffparser_parse(
                self.internal_uffparser,
                CString::new(file_path.to_str().unwrap()).unwrap().as_ptr(),
                network.internal_network,
            )
        };
        Ok(())
    }
}

impl Drop for UffParser {
    fn drop(&mut self) {
        unsafe { uffparser_destroy_uff_parser(self.internal_uffparser) };
    }
}
