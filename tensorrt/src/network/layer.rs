use std::ffi::CStr;
use tensorrt_sys::layer_get_name;

pub struct Layer {
    pub(crate) internal_layer: *mut tensorrt_sys::Layer_t,
}

impl Layer {
    pub fn get_name(&self) -> String {
        let raw_string = unsafe {
            let ptr = layer_get_name(self.internal_layer);
            CStr::from_ptr(ptr)
        };

        raw_string.to_str().unwrap().to_string()
    }
}
