//
// Created by mason on 8/26/19.
//

#ifndef LIBTRT_TRTCUDAENGINE_H
#define LIBTRT_TRTCUDAENGINE_H

#include <NvInfer.h>

#include "../TRTContext/TRTContext.h"
#include "../TRTHostMemory/TRTHostMemory.h"
#include "../TRTDims/TRTDims.h"
#include "../TRTEnums.h"

void engine_destroy(nvinfer1::ICudaEngine* engine);
nvinfer1::IExecutionContext* engine_create_execution_context(nvinfer1::ICudaEngine *engine);
nvinfer1::IExecutionContext* engine_create_execution_context_without_device_memory(nvinfer1::ICudaEngine *engine);
int engine_get_nb_bindings(nvinfer1::ICudaEngine* engine);
int engine_get_binding_index(nvinfer1::ICudaEngine *engine, const char* op_name);
const char* engine_get_binding_name(nvinfer1::ICudaEngine* engine, int binding_index);
bool engine_binding_is_input(nvinfer1::ICudaEngine *engine, int binding_index);
nvinfer1::Dims engine_get_binding_dimensions(nvinfer1::ICudaEngine *engine, int binding_index);
DataType_t engine_get_binding_data_type(nvinfer1::ICudaEngine *engine, int binding_index);
int engine_get_max_batch_size(nvinfer1::ICudaEngine *engine);
int engine_get_nb_layers(nvinfer1::ICudaEngine *engine);
size_t engine_get_workspace_size(nvinfer1::ICudaEngine *engine);
nvinfer1::IHostMemory* engine_serialize(nvinfer1::ICudaEngine* engine);
TensorLocation_t engine_get_location(nvinfer1::ICudaEngine *engine, int binding_index);
size_t engine_get_device_memory_size(nvinfer1::ICudaEngine *engine);
bool engine_is_refittable(nvinfer1::ICudaEngine *engine);

#endif //LIBTRT_TRTCUDAENGINE_H
