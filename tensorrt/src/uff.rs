use crate::network::Network;
use std::error;
use std::ffi::CString;
use std::fmt::Formatter;
use std::io;
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

    pub fn register_input(&self, input_name: &str) -> Result<(), UFFRegistrationError> {
        let mut d_vec = vec![3, 256, 256];

        // Could be a better way to model this in Rust and pass it to C.
        // Value represents what type each of dimensions above are which is not well communicated
        // by the [1, 0, 0] that currently exists.
        let mut type_vec = vec![1, 0, 0];
        let dims = Dims {
            nbDims: 3,
            d: d_vec.as_mut_ptr(),
            type_: type_vec.as_mut_ptr(),
        };
        let res = unsafe {
            uffparser_register_input(
                self.internal_uffparser,
                CString::new(input_name).unwrap().as_ptr(),
                dims,
            )
        };

        if res {
            Ok(())
        } else {
            Err(UFFRegistrationError::new("Input Registration Failed"))
        }
    }

    pub fn register_output(&self, output_name: &str) -> Result<(), UFFRegistrationError> {
        let res = unsafe {
            uffparser_register_output(
                self.internal_uffparser,
                CString::new(output_name).unwrap().as_ptr(),
            )
        };

        if res {
            Ok(())
        } else {
            Err(UFFRegistrationError::new("Output Registration Failed"))
        }
    }

    pub fn parse(&self, file_path: &Path, network: &Network) -> Result<(), std::io::Error> {
        if !file_path.exists() {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                "UFF file does not exist",
            ));
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

#[derive(Debug, Clone)]
pub struct UFFRegistrationError {
    message: String,
}

impl UFFRegistrationError {
    pub fn new(message: &str) -> Self {
        UFFRegistrationError {
            message: message.to_string(),
        }
    }
}

impl std::fmt::Display for UFFRegistrationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl error::Error for UFFRegistrationError {}
