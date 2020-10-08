//
// Created by mason on 11/27/19.
//
#include <memory>
#include <NvInfer.h>
#include <NvInferPlugin.h>

#include "TRTBuilder.h"
#include "../TRTNetworkDefinition/TRTNetworkDefinitionInternal.hpp"
#include "../TRTLogger/TRTLoggerInternal.hpp"
#include "../TRTCudaEngine/TRTCudaEngineInternal.hpp"
#include "../TRTLayer/TRTLayerInternal.hpp"
#include "../TRTUtils.hpp"

#define MAX_WORKSPACE (1 << 30)

struct Builder {
    using IBuilderPtr = std::unique_ptr<nvinfer1::IBuilder, TRTDeleter<nvinfer1::IBuilder>>;
    IBuilderPtr internal_builder;

    explicit Builder(nvinfer1::ILogger& logger) {
        internal_builder = IBuilderPtr(nvinfer1::createInferBuilder(logger));
        internal_builder->setMaxBatchSize(1);
        internal_builder->setMaxWorkspaceSize(MAX_WORKSPACE);
    };
};


void builder_set_max_batch_size(Builder_t* builder, int32_t batch_size) {
    if (builder == nullptr) {
        return;
    }

    builder->internal_builder->setMaxBatchSize(batch_size);
}

int32_t builder_get_max_batch_size(Builder_t* builder) {
    if (builder == nullptr)
        return -1;

    return builder->internal_builder->getMaxBatchSize();
}

void builder_set_max_workspace_size(Builder_t* builder, size_t workspace_size) {
    if (builder == nullptr)
        return;

    builder->internal_builder->setMaxWorkspaceSize(workspace_size);
}

size_t builder_get_max_workspace_size(Builder_t* builder) {
    if (builder == nullptr)
        return -1;

    return builder->internal_builder->getMaxWorkspaceSize();
}

void builder_set_half2_mode(Builder_t* builder, bool mode) {
    if(builder == nullptr)
        return;

    builder->internal_builder->setHalf2Mode(mode);
}

bool builder_get_half2_mode(Builder_t* builder) {
    if(builder == nullptr)
        return false;

    return builder->internal_builder->getHalf2Mode();
}

void builder_set_debug_sync(Builder_t* builder, bool sync) {
    if(builder == nullptr)
        return;

    builder->internal_builder->setDebugSync(sync);
}

bool builder_get_debug_sync(Builder_t* builder) {
    if(builder == nullptr)
        return false;

    return builder->internal_builder->getDebugSync();
}

void builder_set_min_find_iterations(Builder_t* builder, int min_find) {
    if(builder == nullptr)
        return;

    builder->internal_builder->setMinFindIterations(min_find);
}

int builder_get_min_find_iterations(Builder_t* builder) {
    if(builder == nullptr)
        return -1;

    return builder->internal_builder->getMinFindIterations();
}

void builder_set_average_find_iterations(Builder_t* builder, int avg_find) {
    if (builder == nullptr)
        return;

    builder->internal_builder->setAverageFindIterations(avg_find);
}

int builder_get_average_find_iterations(Builder_t* builder) {
    if(builder == nullptr)
        return -1;

    return builder->internal_builder->getAverageFindIterations();
}

bool builder_platform_has_fast_fp16(Builder_t* builder){
    if(builder == nullptr)
        return false;

    return builder->internal_builder->platformHasFastFp16();
}

bool builder_platform_has_fast_int8(Builder_t* builder) {
    if(builder == nullptr)
        return false;

    return builder->internal_builder->platformHasFastInt8();
}

void builder_set_int8_mode(Builder_t* builder, bool mode) {
    if(builder == nullptr)
        return;

    builder->internal_builder->setInt8Mode(mode);
}

bool builder_get_int8_mode(Builder_t* builder) {
    if(builder == nullptr)
        return false;

    return builder->internal_builder->getInt8Mode();
}

void builder_set_fp16_mode(Builder_t* builder, bool mode) {
    if (builder == nullptr)
        return;

    builder->internal_builder->setFp16Mode(mode);
}

bool builder_get_fp16_mode(Builder_t* builder) {
    if (builder == nullptr)
        return false;

    return builder->internal_builder->getFp16Mode();
}

void builder_set_device_type(Builder_t* builder, Layer_t* layer, DeviceType_t deviceType) {
    if(builder == nullptr || layer == nullptr) {
        return;
    }

    builder->internal_builder->setDeviceType(layer->internal_layer, static_cast<nvinfer1::DeviceType>(deviceType));
}

DeviceType_t builder_get_device_type(Builder_t* builder, Layer_t* layer) {
    if (builder == nullptr) {
        return DeviceType::kGPU;
    }

    return static_cast<DeviceType_t>(builder->internal_builder->getDeviceType(layer->internal_layer));
}

bool builder_is_device_type_set(Builder_t* builder, Layer_t* layer) {
    if (builder == nullptr || layer == nullptr) {
        return false;
    }

    return builder->internal_builder->isDeviceTypeSet(layer->internal_layer);
}

void builder_set_default_device_type(Builder_t* builder, DeviceType_t deviceType) {
    if(builder == nullptr)
        return;

    builder->internal_builder->setDefaultDeviceType(static_cast<nvinfer1::DeviceType>(deviceType));
}

DeviceType_t builder_get_default_device_type(Builder_t *builder) {
    if (builder == nullptr)
        return DeviceType::kGPU;

    return static_cast<DeviceType_t>(builder->internal_builder->getDefaultDeviceType());
}

void builder_reset_device_type(Builder_t* builder, Layer_t* layer) {
   if (builder == nullptr || layer == nullptr) {
       return;
   }

   builder->internal_builder->resetDeviceType(layer->internal_layer);
}

bool builder_can_run_on_dla(Builder_t* builder, Layer_t* layer) {
    if (builder == nullptr || layer == nullptr) {
        return false;
    }

    return builder->internal_builder->canRunOnDLA(layer->internal_layer);
}

int builder_get_max_dla_batch_size(Builder_t* builder) {
    if(builder == nullptr)
        return -1;

    return builder->internal_builder->getMaxBatchSize();
}

void builder_allow_gpu_fallback(Builder_t* builder, bool set_fallback_mode) {
    if(builder == nullptr)
        return;

    builder->internal_builder->allowGPUFallback(set_fallback_mode);
}

int builder_get_nb_dla_cores(Builder_t* builder) {
    if (builder == nullptr)
        return -1;

    return builder->internal_builder->getNbDLACores();
}

void builder_set_dla_core(Builder_t* builder, int dla_core) {
    if(builder == nullptr)
        return;

    builder->internal_builder->setDLACore(dla_core);
}

int builder_get_dla_core(Builder_t* builder) {
    if(builder == nullptr)
        return -1;

    return builder->internal_builder->getDLACore();
}

void builder_set_strict_type_constraints(Builder_t* builder, bool mode) {
    if(builder == nullptr)
        return;

    builder->internal_builder->setStrictTypeConstraints(mode);
}

bool builder_get_strict_type_constraints(Builder_t* builder) {
    if(builder == nullptr)
        return false;

    return builder->internal_builder->getStrictTypeConstraints();
}

void builder_set_refittable(Builder_t* builder, bool can_refit) {
    if(builder == nullptr)
        return;

    builder->internal_builder->setRefittable(can_refit);
}

bool builder_get_refittable(Builder_t* builder) {
    if (builder == nullptr)
        return false;

    return builder->internal_builder->getRefittable();
}

void builder_set_engine_capability(Builder_t* builder, EngineCapabiliy_t engine_capability) {
    if (builder == nullptr)
        return;

    builder->internal_builder->setEngineCapability(static_cast<nvinfer1::EngineCapability>(engine_capability));
}

EngineCapabiliy_t builder_get_engine_capability(Builder_t* builder) {
    if (builder == nullptr)
        return EngineCapabiliy::kDEFAULT;

    return static_cast<EngineCapabiliy_t>(builder->internal_builder->getEngineCapability());
}

Builder_t *create_infer_builder(Logger_t *logger) {
    initLibNvInferPlugins(&logger->getLogger(), "");

    return new Builder(logger->getLogger());
}

void destroy_builder(Builder_t *builder) {
    if (builder == nullptr)
        return;

    delete builder;
}

Network_t *create_network(Builder_t *builder) {
    if (builder == nullptr)
        return nullptr;

    return new Network(builder->internal_builder->createNetwork());
}

#if defined(TRT6) || defined(TRT7)
Network_t *create_network_v2(Builder_t *builder, uint32_t flags) {
    if (builder == nullptr)
        return nullptr;

    return new Network(builder->internal_builder->createNetworkV2(flags));
}
#endif

Engine_t *build_cuda_engine(Builder_t *builder, Network_t *network) {
    if (builder == nullptr || network == nullptr)
        return nullptr;

    auto& b = builder->internal_builder;

    auto engine = b->buildCudaEngine(network->getNetworkDefinition());
    return create_engine(engine);
}

void builder_reset(Builder_t* builder, Network_t* network) {
    if(builder == nullptr || network == nullptr)
        return;

    builder->internal_builder->reset(network->getNetworkDefinition());
}
