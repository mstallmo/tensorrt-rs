use super::*;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use tensorrt_rs_derive::Layer;
use tensorrt_sys::pooling_destroy;

#[repr(C)]
#[derive(FromPrimitive, Debug, Eq, PartialEq)]
pub enum PoolingType {
    Max,
    Average,
    MaxAverageBlend,
}

#[derive(Layer)]
pub struct PoolingLayer {
    pub(crate) internal_layer: *mut tensorrt_sys::Layer_t,
}

impl Drop for PoolingLayer {
    fn drop(&mut self) {
        unsafe { pooling_destroy(self.internal_layer) }
    }
}
