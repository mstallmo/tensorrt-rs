use std::marker::PhantomData;

use crate::engine::Engine;
use crate::network::Network;
use crate::runtime::Logger;
#[cfg(not(feature = "trt-5"))]
use tensorrt_sys::create_network_v2;
use tensorrt_sys::{
    build_cuda_engine, builder_get_max_batch_size, builder_get_max_workspace_size,
    builder_set_max_batch_size, builder_set_max_workspace_size, create_infer_builder,
    create_network, destroy_builder,
};

pub struct Builder<'a> {
    pub(crate) internal_builder: *mut tensorrt_sys::Builder_t,
    pub(crate) logger: PhantomData<&'a Logger>,
}

bitflags! {
    pub struct NetworkBuildFlags: u32 {
        const EXPLICIT_BATCH = 0b1;
        const EXPLICIT_PRECISION = 0b10;
    }
}

impl<'a> Builder<'a> {
    pub fn new(logger: &'a Logger) -> Self {
        let internal_builder = unsafe { create_infer_builder(logger.internal_logger) };
        let logger = PhantomData;
        Self {
            internal_builder,
            logger,
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

    pub fn create_network(&self) -> Network {
        let internal_network = unsafe { create_network(self.internal_builder) };
        Network { internal_network }
    }

    #[cfg(not(feature = "trt-5"))]
    pub fn create_network_v2(&self, flags: NetworkBuildFlags) -> Network {
        let internal_network = unsafe { create_network_v2(self.internal_builder, flags.bits()) };
        Network { internal_network }
    }

    pub fn build_cuda_engine(&self, network: &Network) -> Engine<'a> {
        let internal_engine =
            unsafe { build_cuda_engine(self.internal_builder, network.internal_network) };
        let logger = self.logger;
        Engine {
            internal_engine,
            logger,
        }
    }
}

impl<'a> Drop for Builder<'a> {
    fn drop(&mut self) {
        unsafe { destroy_builder(self.internal_builder) };
    }
}
