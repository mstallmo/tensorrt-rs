use tensorrt_sys::destroy_network;

pub struct Network {
    pub(crate) internal_network: *mut tensorrt_sys::Network_t,
}

impl Drop for Network {
    fn drop(&mut self) {
        unsafe { destroy_network(self.internal_network) };
    }
}
