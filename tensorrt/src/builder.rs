use crate::network::Network;
use crate::runtime::Logger;
use tensorrt_sys::{build_cuda_engine, create_infer_builder, create_network, destroy_builder};

pub struct Builder {
    internal_builder: *mut tensorrt_sys::Builder_t,
}

impl Builder {
    pub fn new(logger: &Logger) -> Builder {
        let builder = unsafe { create_infer_builder(logger.internal_logger) };
        Builder {
            internal_builder: builder,
        }
    }

    pub fn create_network(&self) -> Network {
        let network = unsafe { create_network(self.internal_builder) };
        Network {
            internal_network: network,
        }
    }

    pub fn build_cuda_engine(&self, network: &Network) -> *mut tensorrt_sys::Engine_t {
        unsafe { build_cuda_engine(self.internal_builder, network.internal_network) }
    }
}

impl Drop for Builder {
    fn drop(&mut self) {
        unsafe { destroy_builder(self.internal_builder) };
    }
}
