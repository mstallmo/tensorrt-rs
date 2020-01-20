//
// Created by mason on 8/26/19.
//
#include <cstdlib>

#include "NvInfer.h"
#include "TRTCudaEngine.h"

struct Engine {
    void* internal_engine;
};

Engine_t* create_engine(void* engine) {
    Engine_t* e;
    e = (typeof(e))malloc(sizeof(e));
    e->internal_engine = engine;
    return e;
}

void destroy_cuda_engine(Engine_t* engine) {
    if (engine == nullptr)
        return;

    auto en = static_cast<nvinfer1::ICudaEngine* >(engine->internal_engine);
    en->destroy();
    free(engine);
}

int get_nb_bindings(Engine_t* engine) {
    if (engine == nullptr)
        return -1;

    auto en = static_cast<nvinfer1::ICudaEngine* >(engine->internal_engine);
    return en->getNbBindings();
}

const char* get_binding_name(Engine_t* engine, int binding_index) {
    if (engine == nullptr)
        return "";

    auto en = static_cast<nvinfer1::ICudaEngine* >(engine->internal_engine);
    return en->getBindingName(binding_index);
}

int get_binding_index(Engine_t* engine, const char* op_name) {
    if (engine == nullptr)
        return -1;

    auto en = static_cast<nvinfer1::ICudaEngine *>(engine->internal_engine);
    return en->getBindingIndex(op_name);
}

Context_t* engine_create_execution_context(Engine_t* engine) {
    if (engine == nullptr)
        return nullptr;

    auto en = static_cast<nvinfer1::ICudaEngine *>(engine->internal_engine);
    nvinfer1::IExecutionContext *context = en->createExecutionContext();
    return create_execution_context(context);
}

HostMemory_t* engine_serialize(Engine_t* engine) {
    if (engine == nullptr)
        return nullptr;

    auto en = static_cast<nvinfer1::ICudaEngine *>(engine->internal_engine);
    nvinfer1::IHostMemory* hostMemory = en->serialize();
    return create_host_memory(hostMemory);
}
