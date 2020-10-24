#[cfg(test)]
mod tests;

use std::marker::PhantomData;

use crate::engine::Engine;
use crate::network::layer::Layer;
use crate::network::Network;
use crate::runtime::Logger;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use std::os::raw::c_int;
#[cfg(feature = "trt-5")]
use tensorrt_sys::create_network;
#[cfg(not(feature = "trt-5"))]
use tensorrt_sys::create_network_v2;

use tensorrt_sys::{
    build_cuda_engine, builder_allow_gpu_fallback, builder_can_run_on_dla,
    builder_get_average_find_iterations, builder_get_debug_sync, builder_get_default_device_type,
    builder_get_device_type, builder_get_dla_core, builder_get_engine_capability,
    builder_get_fp16_mode, builder_get_half2_mode, builder_get_int8_mode,
    builder_get_max_batch_size, builder_get_max_dla_batch_size, builder_get_max_workspace_size,
    builder_get_min_find_iterations, builder_get_nb_dla_cores, builder_get_refittable,
    builder_get_strict_type_constraints, builder_is_device_type_set,
    builder_platform_has_fast_fp16, builder_platform_has_fast_int8, builder_reset,
    builder_reset_device_type, builder_set_average_find_iterations, builder_set_debug_sync,
    builder_set_default_device_type, builder_set_device_type, builder_set_dla_core,
    builder_set_engine_capability, builder_set_fp16_mode, builder_set_half2_mode,
    builder_set_int8_mode, builder_set_max_batch_size, builder_set_max_workspace_size,
    builder_set_min_find_iterations, builder_set_refittable, builder_set_strict_type_constraints,
    create_infer_builder, destroy_builder,
};

#[repr(C)]
#[derive(Eq, PartialEq, Debug, FromPrimitive)]
pub enum DeviceType {
    GPU,
    DLA,
}

#[repr(C)]
#[derive(Eq, PartialEq, Debug, FromPrimitive)]
pub enum EngineCapability {
    Default,
    SafeGpu,
    SafeDla,
}

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

    pub fn set_device_type<T: Layer>(&self, layer: &T, device_type: DeviceType) {
        unsafe {
            builder_set_device_type(
                self.internal_builder,
                layer.get_internal_layer(),
                device_type as c_int,
            )
        }
    }

    pub fn get_device_type(&self, layer: &dyn Layer) -> DeviceType {
        let primitive =
            unsafe { builder_get_device_type(self.internal_builder, layer.get_internal_layer()) };
        FromPrimitive::from_i32(primitive).unwrap()
    }

    pub fn is_device_type_set(&self, layer: &dyn Layer) -> bool {
        unsafe { builder_is_device_type_set(self.internal_builder, layer.get_internal_layer()) }
    }

    pub fn set_default_device_type(&self, device_type: DeviceType) {
        unsafe { builder_set_default_device_type(self.internal_builder, device_type as c_int) }
    }

    pub fn get_default_device_type(&self) -> DeviceType {
        let primitive = unsafe { builder_get_default_device_type(self.internal_builder) };
        FromPrimitive::from_i32(primitive).unwrap()
    }

    pub fn reset_device_type(&self, layer: &dyn Layer) {
        unsafe { builder_reset_device_type(self.internal_builder, layer.get_internal_layer()) }
    }

    pub fn can_run_on_dla(&self, layer: &dyn Layer) -> bool {
        unsafe { builder_can_run_on_dla(self.internal_builder, layer.get_internal_layer()) }
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

    pub fn set_engine_capability(&self, engine_capability: EngineCapability) {
        unsafe { builder_set_engine_capability(self.internal_builder, engine_capability as c_int) }
    }

    pub fn get_engine_capability(&self) -> EngineCapability {
        let primitive = unsafe { builder_get_engine_capability(self.internal_builder) };
        FromPrimitive::from_i32(primitive).unwrap()
    }

    #[cfg(feature = "trt-5")]
    pub fn create_network(&self) -> Network {
        let internal_network = unsafe { create_network(self.internal_builder) };
        Network { internal_network }
    }

    #[cfg(not(feature = "trt-5"))]
    pub fn create_network_v2(&self, flags: NetworkBuildFlags) -> Network {
        let internal_network = unsafe { create_network_v2(self.internal_builder, flags.bits()) };
        Network { internal_network }
    }

    pub fn build_cuda_engine(&self, network: &Network) -> Engine {
        let internal_engine =
            unsafe { build_cuda_engine(self.internal_builder, network.internal_network) };
        Engine { internal_engine }
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
