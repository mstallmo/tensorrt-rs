use crate::engine::Engine;
use crate::network::Network;
use crate::runtime::Logger;
use tensorrt_sys::{build_cuda_engine, create_infer_builder, create_network, destroy_builder};

pub struct Builder {
    internal_builder: *mut tensorrt_sys::Builder_t,
    network: Network,
}

impl Builder {
    pub fn new(logger: &Logger) -> Builder {
        let builder = unsafe { create_infer_builder(logger.internal_logger) };
        let sys_network = unsafe { create_network(builder) };
        let network = Network {
            internal_network: sys_network,
        };

        Builder {
            internal_builder: builder,
            network,
        }
    }

    pub fn get_network(&self) -> &Network {
        &self.network
    }

    pub fn build_cuda_engine(&self) -> Engine {
        Engine {
            internal_engine: unsafe {
                build_cuda_engine(self.internal_builder, self.network.internal_network)
            },
        }
    }
}

impl Drop for Builder {
    fn drop(&mut self) {
        unsafe { destroy_builder(self.internal_builder) };
    }
}
