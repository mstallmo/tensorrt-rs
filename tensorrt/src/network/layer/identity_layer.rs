use super::*;
use tensorrt_rs_derive::Layer;
use tensorrt_sys::nvinfer1_IIdentityLayer;

#[derive(Layer)]
pub struct IdentityLayer {
    pub(crate) internal_layer: *mut nvinfer1_IIdentityLayer,
}
