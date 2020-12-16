//
// Created by mason on 8/26/19.
//
#include <cstring>

#include "TRTCudaEngine.h"

void engine_destroy(nvinfer1::ICudaEngine* engine) {
    engine->destroy();
}

int engine_get_nb_bindings(nvinfer1::ICudaEngine* engine) {
    return engine->getNbBindings();
}

int engine_get_binding_index(nvinfer1::ICudaEngine* engine, const char* op_name) {
    return engine->getBindingIndex(op_name);
}

const char* engine_get_binding_name(nvinfer1::ICudaEngine* engine, int binding_index) {
    return engine->getBindingName(binding_index);
}

bool engine_binding_is_input(nvinfer1::ICudaEngine *engine, int binding_index) {
    return engine->bindingIsInput(binding_index);
}

nvinfer1::Dims engine_get_binding_dimensions(nvinfer1::ICudaEngine *engine, int binding_index) {
    return engine->getBindingDimensions(binding_index);
}

DataType_t engine_get_binding_data_type(nvinfer1::ICudaEngine *engine, int binding_index) {
    return static_cast<DataType_t>(engine->getBindingDataType(binding_index));
}

int engine_get_max_batch_size(nvinfer1::ICudaEngine *engine) {
    return engine->getMaxBatchSize();
}

int engine_get_nb_layers(nvinfer1::ICudaEngine *engine) {
    return engine->getNbLayers();
}

size_t engine_get_workspace_size(nvinfer1::ICudaEngine *engine) {
    return engine->getWorkspaceSize();
}

nvinfer1::IExecutionContext* engine_create_execution_context(nvinfer1::ICudaEngine *engine) {
    return engine->createExecutionContext();
}

nvinfer1::IExecutionContext* engine_create_execution_context_without_device_memory(nvinfer1::ICudaEngine *engine) {
    return engine->createExecutionContextWithoutDeviceMemory();
}

nvinfer1::IHostMemory* engine_serialize(nvinfer1::ICudaEngine* engine) {
    return engine->serialize();
}

TensorLocation_t  engine_get_location(nvinfer1::ICudaEngine *engine, int binding_index) {
    return static_cast<TensorLocation_t>(engine->getLocation(binding_index));
}

size_t engine_get_device_memory_size(nvinfer1::ICudaEngine *engine) {
    return engine->getDeviceMemorySize();
}

bool engine_is_refittable(nvinfer1::ICudaEngine *engine) {
    return engine->isRefittable();
}
