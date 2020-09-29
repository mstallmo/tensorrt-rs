use crate::dims::IsDim;
use tensorrt_sys::{destroy_network, network_get_input, tensor_set_dimensions};

pub struct Network {
    pub(crate) internal_network: *mut tensorrt_sys::Network_t,
}

pub struct Tensor {
    pub(crate) internal_tensor: *mut tensorrt_sys::Tensor_t,
}

impl Network {
    pub fn get_input(&self, idx: i32) -> Tensor {
        let internal_tensor = unsafe { network_get_input(self.internal_network, idx) };
        Tensor { internal_tensor }
    }
}

impl Tensor {
    pub fn set_dimensions<D: IsDim>(&mut self, dims: D) {
        unsafe { tensor_set_dimensions(self.internal_tensor, dims.internal_dims()) };
    }
}

impl Drop for Network {
    fn drop(&mut self) {
        unsafe { destroy_network(self.internal_network) };
    }
}
