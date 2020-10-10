//
// Created by mason on 9/17/19.
//
#include <iostream>
#include <memory>
#include <cuda_runtime.h>
#include "NvInfer.h"
#include "TRTContextInternal.hpp"
#include "../TRTProfiler/TRTProfilerInternal.hpp"
#include "../TRTUtils.hpp"

struct Context {
    using IExecutionContextPtr = std::unique_ptr<nvinfer1::IExecutionContext, TRTDeleter<nvinfer1::IExecutionContext>>;
    IExecutionContextPtr internal_context;

    explicit Context(nvinfer1::IExecutionContext *executionContext) {
        internal_context = IExecutionContextPtr(executionContext);
    }

    ~Context() {
        if (_concreteProfiler) {
            _concreteProfiler->destroy();
        }
    }

    ConcreteProfiler* _concreteProfiler = nullptr;
};

Context_t *create_execution_context(nvinfer1::IExecutionContext *execution_context) {
    return new Context(execution_context);
}

void destroy_excecution_context(Context_t *execution_context) {
    if (execution_context == nullptr)
        return;

    delete execution_context;
}

void context_set_debug_sync(Context_t *execution_context, bool sync) {
    execution_context->internal_context->setDebugSync(sync);
}

bool context_get_debug_sync(Context_t *execution_context) {
    return execution_context->internal_context->getDebugSync();
}

void context_set_name(Context_t *execution_context, const char *name) {
    if (execution_context == nullptr)
        return;

    execution_context->internal_context->setName(name);
}

const char *context_get_name(Context_t *execution_context) {
    if (execution_context == nullptr)
        return "";

    return execution_context->internal_context->getName();
}

void context_set_profiler(Context_t *context, Profiler_t *profiler) {
    auto concreteProfiler = new ConcreteProfiler(profiler);
    context->_concreteProfiler = concreteProfiler;
    context->internal_context->setProfiler(concreteProfiler);
}

Profiler_t* context_get_profiler(Context_t *context) {
    auto concreteProfiler = dynamic_cast<ConcreteProfiler *>(context->internal_context->getProfiler());
    return concreteProfiler->getInternalProfiler();
}

void execute(const Context_t *execution_context, const void **binding_data, const int num_bindings,
             const size_t *data_sizes) {
    if (execution_context == nullptr)
        return;

    auto &context = execution_context->internal_context;
    int nbBindings = context->getEngine().getNbBindings();
    void *buffers[nbBindings];
    for (int i = 0; i < nbBindings; ++i) {
        cudaMalloc(&buffers[i], data_sizes[i]);
    }

    cudaMemcpy(buffers[0], binding_data[0], data_sizes[0], cudaMemcpyHostToDevice);
    context->execute(1, &buffers[0]);

    for (int i = 1; i < num_bindings; ++i) {
        cudaMemcpy((void *)binding_data[i], buffers[i], data_sizes[i], cudaMemcpyDeviceToHost);
        cudaFree(buffers[i]);
    }
}