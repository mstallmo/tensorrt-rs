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

#ifdef __cplusplus
extern "C" {
#endif

struct Builder;
typedef struct Builder Builder_t;

Builder_t *create_infer_builder(Logger_t *logger);
void destroy_builder(Builder_t* builder);

void builder_set_max_batch_size(Builder_t* builder, int32_t batch_size);
int32_t builder_get_max_batch_size(Builder_t* builder);

void builder_set_max_workspace_size(Builder_t* builder, size_t batch_size);
size_t builder_get_max_workspace_size(Builder_t* builder);

void builder_set_half2_mode(Builder_t* builder, bool mode);
bool builder_get_half2_mode(Builder_t* builder);

void builder_set_debug_sync(Builder_t* builder, bool sync);
bool builder_get_debug_sync(Builder_t* builder);

void builder_set_min_find_iterations(Builder_t* builder, int min_find);
int builder_get_min_find_iterations(Builder_t* builder);

void builder_set_average_find_iterations(Builder_t* builder, int avg_find);
int builder_get_average_find_iterations(Builder_t* builder);

bool builder_platform_has_fast_fp16(Builder_t* builder);
bool builder_platform_has_fast_int8(Builder_t* builder);

void builder_set_int8_mode(Builder_t* builder, bool mode);
bool builder_get_int8_mode(Builder_t* builder);

void builder_set_fp16_mode(Builder_t* builder, bool mode);
bool builder_get_fp16_mode(Builder_t* builder);

void builder_set_device_type(Builder_t* builder, Layer_t* layer, DeviceType_t deviceType);
DeviceType_t  builder_get_device_type(Builder_t* builder, Layer_t* layer);
bool builder_is_device_type_set(Builder_t* builder, Layer_t* layer);

void builder_set_default_device_type(Builder_t* builder, DeviceType_t deviceType);
DeviceType_t  builder_get_default_device_type(Builder_t* builder);

void builder_reset_device_type(Builder_t* builder, Layer_t* layer);
bool builder_can_run_on_dla(Builder_t* builder, Layer_t* layer);

int builder_get_max_dla_batch_size(Builder_t* builder);

void builder_allow_gpu_fallback(Builder_t* builder, bool set_fallback_mode);

int builder_get_nb_dla_cores(Builder_t* builder);
void builder_set_dla_core(Builder_t* builder, int dla_core);
int builder_get_dla_core(Builder_t* builder);

void builder_set_strict_type_constraints(Builder_t* builder, bool mode);
bool builder_get_strict_type_constraints(Builder_t* builder);

void builder_set_refittable(Builder_t* builder, bool can_refit);
bool builder_get_refittable(Builder_t* builder);

void builder_set_engine_capability(Builder_t* builder, EngineCapabiliy_t engine_capability);
EngineCapabiliy_t builder_get_engine_capability(Builder_t* builder);

Network_t *create_network(Builder_t* builder);

#if defined(TRT6) || defined(TRT7)
Network_t *create_network_v2(Builder_t* builder, uint32_t flags);
#endif

void builder_reset(Builder_t* builder, Network_t* network);

#ifdef __cplusplus
};
#endif

nvinfer1::ICudaEngine *build_cuda_engine(Builder_t *builder, Network_t *network);

#endif //LIBTRT_TRTBUILDER_H
