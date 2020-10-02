use crate::network::Network;
use crate::runtime::Logger;
use std::error;
use std::ffi::CString;
use std::fmt::Formatter;
use std::io;
use std::marker::PhantomData;
use std::path::{Path, PathBuf};
use tensorrt_sys::{
    onnxparser_create_parser, onnxparser_destroy_parser, onnxparser_parse_from_file,
};

pub struct OnnxFile(PathBuf);

impl OnnxFile {
    pub fn new(file_name: &Path) -> Result<OnnxFile, io::Error> {
        if !file_name.exists() {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                "ONNX file does not exist",
            ));
        }

        if file_name.extension().unwrap() != "onnx" {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Invalid ONNX file. ONNX files should have a .onnx ending",
            ));
        }

        Ok(OnnxFile(file_name.to_path_buf()))
    }

    pub fn path(&self) -> CString {
        CString::new(self.0.to_str().unwrap()).unwrap()
    }
}

pub struct OnnxParser<'a, 'b> {
    internal_onnxparser: *mut tensorrt_sys::OnnxParser_t,
    pub(crate) network: PhantomData<&'a Network>,
    pub(crate) logger: PhantomData<&'b Logger>,
}

impl<'a, 'b> OnnxParser<'a, 'b> {
    // const Network_t *network, Logger_t *logger
    pub fn new(network: &'a Network, logger: &'b Logger) -> Self {
        let internal_onnxparser =
            unsafe { onnxparser_create_parser(network.internal_network, logger.internal_logger) };
        Self {
            internal_onnxparser,
            network: PhantomData,
            logger: PhantomData,
        }
    }

    pub fn parse_from_file(
        &self,
        onnx_file: &OnnxFile,
        verbosity: i32,
    ) -> Result<(), OnnxParseError> {
        let res = unsafe {
            onnxparser_parse_from_file(
                self.internal_onnxparser,
                onnx_file.path().as_ptr(),
                verbosity,
            )
        };

        if res {
            Ok(())
        } else {
            Err(OnnxParseError::new("Error parsing ONNX file"))
        }
    }
}

impl<'a, 'b> Drop for OnnxParser<'a, 'b> {
    fn drop(&mut self) {
        unsafe { onnxparser_destroy_parser(self.internal_onnxparser) };
    }
}

#[derive(Debug, Clone)]
pub struct OnnxParseError {
    message: String,
}

impl OnnxParseError {
    pub fn new(message: &str) -> Self {
        Self {
            message: message.to_string(),
        }
    }
}

impl std::fmt::Display for OnnxParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl error::Error for OnnxParseError {}
