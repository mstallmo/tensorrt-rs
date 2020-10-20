use super::*;
use tensorrt_rs_derive::Layer;

#[derive(Layer)]
pub struct IdentityLayer {
    pub(crate) internal_layer: *mut tensorrt_sys::Layer_t,
}
