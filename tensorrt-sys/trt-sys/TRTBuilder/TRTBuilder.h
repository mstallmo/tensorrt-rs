//
// Created by mason on 11/27/19.
//

#ifndef LIBTRT_TRTBUILDER_H
#define LIBTRT_TRTBUILDER_H

#include <NvInfer.h>
#include "../TRTLogger/TRTLogger.h"
#include "../TRTNetworkDefinition/TRTNetworkDefinition.h"
#include "../TRTCudaEngine/TRTCudaEngine.h"
#include "../TRTLayer/TRTLayer.h"
#include "../TRTEnums.h"

#include <stddef.h>
#include <stdint.h>

nvinfer1::ICudaEngine *build_cuda_engine(nvinfer1::IBuilder *builder, nvinfer1::INetworkDefinition *network);
nvinfer1::IBuilder *create_infer_builder(Logger_t *logger);
void destroy_builder(nvinfer1::IBuilder* builder);
void builder_set_max_batch_size(nvinfer1::IBuilder* builder, int32_t batch_size);
int32_t builder_get_max_batch_size(nvinfer1::IBuilder* builder);
void builder_set_max_workspace_size(nvinfer1::IBuilder* builder, size_t batch_size);
size_t builder_get_max_workspace_size(nvinfer1::IBuilder* builder);
void builder_set_half2_mode(nvinfer1::IBuilder* builder, bool mode);
bool builder_get_half2_mode(nvinfer1::IBuilder* builder);
void builder_set_debug_sync(nvinfer1::IBuilder* builder, bool sync);
bool builder_get_debug_sync(nvinfer1::IBuilder* builder);
void builder_set_min_find_iterations(nvinfer1::IBuilder* builder, int min_find);
int builder_get_min_find_iterations(nvinfer1::IBuilder* builder);
void builder_set_average_find_iterations(nvinfer1::IBuilder* builder, int avg_find);
int builder_get_average_find_iterations(nvinfer1::IBuilder* builder);
bool builder_platform_has_fast_fp16(nvinfer1::IBuilder* builder);
bool builder_platform_has_fast_int8(nvinfer1::IBuilder* builder);
void builder_set_int8_mode(nvinfer1::IBuilder* builder, bool mode);
bool builder_get_int8_mode(nvinfer1::IBuilder* builder);
void builder_set_fp16_mode(nvinfer1::IBuilder* builder, bool mode);
bool builder_get_fp16_mode(nvinfer1::IBuilder* builder);
void builder_set_device_type(nvinfer1::IBuilder* builder, Layer_t* layer, DeviceType_t deviceType);
DeviceType_t  builder_get_device_type(nvinfer1::IBuilder* builder, Layer_t* layer);
bool builder_is_device_type_set(nvinfer1::IBuilder* builder, Layer_t* layer);
void builder_set_default_device_type(nvinfer1::IBuilder* builder, DeviceType_t deviceType);
DeviceType_t  builder_get_default_device_type(nvinfer1::IBuilder* builder);
void builder_reset_device_type(nvinfer1::IBuilder* builder, Layer_t* layer);
bool builder_can_run_on_dla(nvinfer1::IBuilder* builder, Layer_t* layer);
int builder_get_max_dla_batch_size(nvinfer1::IBuilder* builder);
void builder_allow_gpu_fallback(nvinfer1::IBuilder* builder, bool set_fallback_mode);
int builder_get_nb_dla_cores(nvinfer1::IBuilder* builder);
void builder_set_dla_core(nvinfer1::IBuilder* builder, int dla_core);
int builder_get_dla_core(nvinfer1::IBuilder* builder);
void builder_set_strict_type_constraints(nvinfer1::IBuilder* builder, bool mode);
bool builder_get_strict_type_constraints(nvinfer1::IBuilder* builder);
void builder_set_refittable(nvinfer1::IBuilder* builder, bool can_refit);
bool builder_get_refittable(nvinfer1::IBuilder* builder);
void builder_set_engine_capability(nvinfer1::IBuilder* builder, EngineCapabiliy_t engine_capability);
EngineCapabiliy_t builder_get_engine_capability(nvinfer1::IBuilder* builder);
#if defined(TRT5)
nvinfer1::INetworkDefinition *create_network(nvinfer1::IBuilder* builder);
#elif defined(TRT6) || defined(TRT7)
nvinfer1::INetworkDefinition *create_network_v2(nvinfer1::IBuilder* builder, uint32_t flags);
#endif

void builder_reset(nvinfer1::IBuilder* builder, nvinfer1::INetworkDefinition* network);
#endif //LIBTRT_TRTBUILDER_H
