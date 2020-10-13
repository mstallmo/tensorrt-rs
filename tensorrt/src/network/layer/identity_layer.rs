use super::private::LayerPrivate;
use super::*;

pub struct IdentityLayer {
    pub(crate) internal_layer: *mut tensorrt_sys::Layer_t,
}

impl Layer for IdentityLayer {}

impl LayerPrivate for IdentityLayer {
    fn get_internal_layer(&self) -> *mut tensorrt_sys::Layer_t {
        self.internal_layer
    }
}
