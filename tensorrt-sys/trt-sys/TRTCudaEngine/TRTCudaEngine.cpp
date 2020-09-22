//
// Created by mason on 8/26/19.
//
#include <memory>
#include <cstring>

#include "../TRTHostMemory/TRTHostMemoryInternal.hpp"
#include "../TRTContext/TRTContextInternal.hpp"
#include "../TRTUtils.hpp"
#include "TRTCudaEngineInternal.hpp"

struct Engine {
    using ICudaEnginePtr = std::unique_ptr<nvinfer1::ICudaEngine, TRTDeleter<nvinfer1::ICudaEngine>>;
    ICudaEnginePtr internal_engine;

    explicit Engine (nvinfer1::ICudaEngine* engine) {
        internal_engine = ICudaEnginePtr(engine);
    }
};

Engine_t* create_engine(nvinfer1::ICudaEngine* engine) {
    return new Engine(engine);
}

void destroy_cuda_engine(Engine_t* engine) {
    if (engine == nullptr)
        return;

    delete engine;
}

int get_nb_bindings(Engine_t* engine) {
    if (engine == nullptr)
        return -1;

    return engine->internal_engine->getNbBindings();
}

const char* get_binding_name(Engine_t* engine, int binding_index) {
    if (engine == nullptr)
        return "";

    return engine->internal_engine->getBindingName(binding_index);
}

int get_binding_index(Engine_t* engine, const char* op_name) {
    if (engine == nullptr)
        return -1;

    return engine->internal_engine->getBindingIndex(op_name);
}

Dims_t* get_binding_dimensions(Engine_t *engine, int binding_index) {
    if (engine == nullptr)
        return nullptr;

    nvinfer1::Dims nvdims = engine->internal_engine->getBindingDimensions(binding_index);
    auto dims = static_cast<Dims_t *>(malloc(sizeof(Dims_t)));
    dims->nbDims = nvdims.nbDims;
    memcpy(dims->d, nvdims.d, nvinfer1::Dims::MAX_DIMS * sizeof(int));
    memcpy(dims->type, nvdims.type, nvinfer1::Dims::MAX_DIMS * sizeof(int));

    return dims;
}

Context_t* engine_create_execution_context(Engine_t* engine) {
    if (engine == nullptr)
        return nullptr;

    nvinfer1::IExecutionContext *context = engine->internal_engine->createExecutionContext();
    return create_execution_context(context);
}

HostMemory_t* engine_serialize(Engine_t* engine) {
    if (engine == nullptr)
        return nullptr;

    return create_host_memory(engine->internal_engine->serialize());
}
