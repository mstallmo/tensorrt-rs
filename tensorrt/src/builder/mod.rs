#[cfg(test)]
mod tests;

use std::marker::PhantomData;

use crate::engine::Engine;
use crate::network::Network;
use crate::runtime::Logger;
#[cfg(not(feature = "trt-5"))]
use tensorrt_sys::create_network_v2;
use tensorrt_sys::{
    build_cuda_engine, builder_allow_gpu_fallback, builder_get_average_find_iterations,
    builder_get_debug_sync, builder_get_dla_core, builder_get_fp16_mode, builder_get_half2_mode,
    builder_get_int8_mode, builder_get_max_batch_size, builder_get_max_dla_batch_size,
    builder_get_max_workspace_size, builder_get_min_find_iterations, builder_get_nb_dla_cores,
    builder_get_refittable, builder_get_strict_type_constraints, builder_platform_has_fast_fp16,
    builder_platform_has_fast_int8, builder_reset, builder_set_average_find_iterations,
    builder_set_debug_sync, builder_set_dla_core, builder_set_fp16_mode, builder_set_half2_mode,
    builder_set_int8_mode, builder_set_max_batch_size, builder_set_max_workspace_size,
    builder_set_min_find_iterations, builder_set_refittable, builder_set_strict_type_constraints,
    create_infer_builder, create_network, destroy_builder,
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

    pub fn set_half2_mode(&self, mode: bool) {
        unsafe { builder_set_half2_mode(self.internal_builder, mode) }
    }

    pub fn get_half2_mode(&self) -> bool {
        unsafe { builder_get_half2_mode(self.internal_builder) }
    }

    pub fn set_debug_sync(&self, sync: bool) {
        unsafe { builder_set_debug_sync(self.internal_builder, sync) }
    }

    pub fn get_debug_sync(&self) -> bool {
        unsafe { builder_get_debug_sync(self.internal_builder) }
    }

    pub fn set_min_find_iterations(&self, min_find: i32) {
        unsafe { builder_set_min_find_iterations(self.internal_builder, min_find) }
    }

    pub fn get_min_find_iterations(&self) -> i32 {
        unsafe { builder_get_min_find_iterations(self.internal_builder) }
    }

    pub fn set_average_find_iterations(&self, avg_find: i32) {
        unsafe { builder_set_average_find_iterations(self.internal_builder, avg_find) }
    }

    pub fn get_average_find_iterations(&self) -> i32 {
        unsafe { builder_get_average_find_iterations(self.internal_builder) }
    }

    pub fn platform_has_fast_fp16(&self) -> bool {
        unsafe { builder_platform_has_fast_fp16(self.internal_builder) }
    }

    pub fn platform_has_fast_int8(&self) -> bool {
        unsafe { builder_platform_has_fast_int8(self.internal_builder) }
    }

    pub fn set_int8_mode(&self, mode: bool) {
        unsafe { builder_set_int8_mode(self.internal_builder, mode) }
    }

    pub fn get_int8_mode(&self) -> bool {
        unsafe { builder_get_int8_mode(self.internal_builder) }
    }

    pub fn set_fp16_mode(&self, mode: bool) {
        unsafe { builder_set_fp16_mode(self.internal_builder, mode) }
    }

    pub fn get_fp16_mode(&self) -> bool {
        unsafe { builder_get_fp16_mode(self.internal_builder) }
    }

    pub fn get_max_dla_batch_size(&self) -> i32 {
        unsafe { builder_get_max_dla_batch_size(self.internal_builder) }
    }

    pub fn allow_gpu_fallback(&self, set_fallback_mode: bool) {
        unsafe { builder_allow_gpu_fallback(self.internal_builder, set_fallback_mode) }
    }

    pub fn get_nb_dla_cores(&self) -> i32 {
        unsafe { builder_get_nb_dla_cores(self.internal_builder) }
    }

    pub fn set_dla_core(&self, dla_core: i32) {
        unsafe { builder_set_dla_core(self.internal_builder, dla_core) }
    }

    pub fn get_dla_core(&self) -> i32 {
        unsafe { builder_get_dla_core(self.internal_builder) }
    }

    pub fn set_strict_type_constraints(&self, mode: bool) {
        unsafe { builder_set_strict_type_constraints(self.internal_builder, mode) }
    }

    pub fn get_strict_type_constraints(&self) -> bool {
        unsafe { builder_get_strict_type_constraints(self.internal_builder) }
    }

    pub fn set_refittable(&self, can_refit: bool) {
        unsafe { builder_set_refittable(self.internal_builder, can_refit) }
    }

    pub fn get_refittable(&self) -> bool {
        unsafe { builder_get_refittable(self.internal_builder) }
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

    pub fn reset(&self, network: Network) {
        unsafe { builder_reset(self.internal_builder, network.internal_network) }
    }
}

impl<'a> Drop for Builder<'a> {
    fn drop(&mut self) {
        unsafe { destroy_builder(self.internal_builder) };
    }
}
