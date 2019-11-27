use crate::network::Network;
use std::ffi::CString;
use tensorrt_sys::{uffparser_create_uff_parser, uffparser_destroy_uff_parser, uffparser_parse};

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

    pub fn parse(self, file_path: &str, network: &Network) {
        unsafe {
            uffparser_parse(
                self.internal_uffparser,
                CString::new(file_path).unwrap().as_ptr(),
                network.internal_network,
            )
        };
    }
}

impl Drop for UffParser {
    fn drop(&mut self) {
        unsafe { uffparser_destroy_uff_parser(self.internal_uffparser) };
    }
}
