use crate::engine::Engine;
use crate::network::Network;
use crate::runtime::Logger;
use tensorrt_sys::{
    build_cuda_engine, builder_get_max_batch_size, builder_get_max_workspace_size,
    builder_set_max_batch_size, builder_set_max_workspace_size, create_infer_builder,
    create_network, create_network_v2, destroy_builder,
};

pub struct Builder {
    internal_builder: *mut tensorrt_sys::Builder_t,
    network: Network,
}

bitflags! {
    pub struct NetworkBuildFlags: u32 {
        const EXPLICIT_BATCH = 0b1;
        const EXPLICIT_PRECISION = 0b10;
    }
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

    pub fn new_v2(logger: &Logger, flags: NetworkBuildFlags) -> Builder {
        let builder = unsafe { create_infer_builder(logger.internal_logger) };
        let sys_network = unsafe { create_network_v2(builder, flags.bits()) };
        let network = Network {
            internal_network: sys_network,
        };

        Builder {
            internal_builder: builder,
            network,
        }
    }

    pub fn get_max_workspace_size(&self) -> usize {
        unsafe { builder_get_max_workspace_size(self.internal_builder) as usize }
    }

    pub fn set_max_workspace_size(&self, ws: usize) {
        unsafe { builder_set_max_workspace_size(self.internal_builder, ws as usize) }
    }

    pub fn get_max_batch_size(&self) -> i32 {
        unsafe { builder_get_max_batch_size(self.internal_builder) as i32 }
    }

    pub fn set_max_batch_size(&self, bs: i32) {
        unsafe { builder_set_max_batch_size(self.internal_builder, bs as i32) }
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
