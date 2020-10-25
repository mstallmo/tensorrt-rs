//
// Created by mason on 11/27/19.
//
#include <memory>
#include <NvInfer.h>
#include <NvInferPlugin.h>

#include "TRTBuilder.h"
#include "../TRTLogger/TRTLoggerInternal.hpp"
#include "../TRTLayer/TRTLayerInternal.hpp"

void builder_set_max_batch_size(nvinfer1::IBuilder* builder, int32_t batch_size) {
    builder->setMaxBatchSize(batch_size);
}

int32_t builder_get_max_batch_size(nvinfer1::IBuilder* builder) {
    return builder->getMaxBatchSize();
}

void builder_set_max_workspace_size(nvinfer1::IBuilder* builder, size_t workspace_size) {
    builder->setMaxWorkspaceSize(workspace_size);
}

size_t builder_get_max_workspace_size(nvinfer1::IBuilder* builder) {
    return builder->getMaxWorkspaceSize();
}

void builder_set_half2_mode(nvinfer1::IBuilder* builder, bool mode) {
    builder->setHalf2Mode(mode);
}

bool builder_get_half2_mode(nvinfer1::IBuilder* builder) {
    return builder->getHalf2Mode();
}

void builder_set_debug_sync(nvinfer1::IBuilder* builder, bool sync) {
    builder->setDebugSync(sync);
}

bool builder_get_debug_sync(nvinfer1::IBuilder* builder) {
    return builder->getDebugSync();
}

void builder_set_min_find_iterations(nvinfer1::IBuilder* builder, int min_find) {
    builder->setMinFindIterations(min_find);
}

int builder_get_min_find_iterations(nvinfer1::IBuilder* builder) {
    return builder->getMinFindIterations();
}

void builder_set_average_find_iterations(nvinfer1::IBuilder* builder, int avg_find) {
    builder->setAverageFindIterations(avg_find);
}

int builder_get_average_find_iterations(nvinfer1::IBuilder* builder) {
    return builder->getAverageFindIterations();
}

bool builder_platform_has_fast_fp16(nvinfer1::IBuilder* builder){
    return builder->platformHasFastFp16();
}

bool builder_platform_has_fast_int8(nvinfer1::IBuilder* builder) {
    return builder->platformHasFastInt8();
}

void builder_set_int8_mode(nvinfer1::IBuilder* builder, bool mode) {
    builder->setInt8Mode(mode);
}

bool builder_get_int8_mode(nvinfer1::IBuilder* builder) {
    return builder->getInt8Mode();
}

void builder_set_fp16_mode(nvinfer1::IBuilder* builder, bool mode) {
    builder->setFp16Mode(mode);
}

bool builder_get_fp16_mode(nvinfer1::IBuilder* builder) {
    return builder->getFp16Mode();
}

void builder_set_device_type(nvinfer1::IBuilder* builder, Layer_t* layer, DeviceType_t deviceType) {
    builder->setDeviceType(layer->internal_layer, static_cast<nvinfer1::DeviceType>(deviceType));
}

DeviceType_t builder_get_device_type(nvinfer1::IBuilder* builder, Layer_t* layer) {
    return static_cast<DeviceType_t>(builder->getDeviceType(layer->internal_layer));
}

bool builder_is_device_type_set(nvinfer1::IBuilder* builder, Layer_t* layer) {
    return builder->isDeviceTypeSet(layer->internal_layer);
}

void builder_set_default_device_type(nvinfer1::IBuilder* builder, DeviceType_t deviceType) {
    builder->setDefaultDeviceType(static_cast<nvinfer1::DeviceType>(deviceType));
}

DeviceType_t builder_get_default_device_type(nvinfer1::IBuilder *builder) {
    return static_cast<DeviceType_t>(builder->getDefaultDeviceType());
}

void builder_reset_device_type(nvinfer1::IBuilder* builder, Layer_t* layer) {
   builder->resetDeviceType(layer->internal_layer);
}

bool builder_can_run_on_dla(nvinfer1::IBuilder* builder, Layer_t* layer) {
    return builder->canRunOnDLA(layer->internal_layer);
}

int builder_get_max_dla_batch_size(nvinfer1::IBuilder* builder) {
    return builder->getMaxBatchSize();
}

void builder_allow_gpu_fallback(nvinfer1::IBuilder* builder, bool set_fallback_mode) {
    builder->allowGPUFallback(set_fallback_mode);
}

int builder_get_nb_dla_cores(nvinfer1::IBuilder* builder) {
    return builder->getNbDLACores();
}

void builder_set_dla_core(nvinfer1::IBuilder* builder, int dla_core) {
    builder->setDLACore(dla_core);
}

int builder_get_dla_core(nvinfer1::IBuilder* builder) {
    return builder->getDLACore();
}

void builder_set_strict_type_constraints(nvinfer1::IBuilder* builder, bool mode) {
    builder->setStrictTypeConstraints(mode);
}

bool builder_get_strict_type_constraints(nvinfer1::IBuilder* builder) {
    return builder->getStrictTypeConstraints();
}

void builder_set_refittable(nvinfer1::IBuilder* builder, bool can_refit) {
    builder->setRefittable(can_refit);
}

bool builder_get_refittable(nvinfer1::IBuilder* builder) {
    return builder->getRefittable();
}

void builder_set_engine_capability(nvinfer1::IBuilder* builder, EngineCapabiliy_t engine_capability) {
    builder->setEngineCapability(static_cast<nvinfer1::EngineCapability>(engine_capability));
}

EngineCapabiliy_t builder_get_engine_capability(nvinfer1::IBuilder* builder) {
    return static_cast<EngineCapabiliy_t>(builder->getEngineCapability());
}

nvinfer1::IBuilder *create_infer_builder(Logger_t *logger) {
    initLibNvInferPlugins(&logger->getLogger(), "");
    return nvinfer1::createInferBuilder(logger->getLogger());
}


void destroy_builder(nvinfer1::IBuilder* builder) {
    builder->destroy();
}

#if defined(TRT5)
nvinfer1::INetworkDefinition *create_network(nvinfer1::IBuilder *builder) {
    return builder->createNetwork();
}
#elif defined(TRT6) || defined(TRT7)
nvinfer1::INetworkDefinition *create_network_v2(nvinfer1::IBuilder *builder, uint32_t flags) {
    return builder->createNetworkV2(flags);
}
#endif

nvinfer1::ICudaEngine *build_cuda_engine(nvinfer1::IBuilder *builder, nvinfer1::INetworkDefinition *network) {
    return builder->buildCudaEngine(*network);
}

void builder_reset(nvinfer1::IBuilder* builder, nvinfer1::INetworkDefinition* network) {
    builder->reset(*network);
}
