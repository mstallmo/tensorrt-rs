//
// Created by mason on 9/17/19.
//
#include <cassert>
#include <cstdlib>
#include <memory>
#include <cuda_runtime.h>
#include "NvInfer.h"
#include "TRTContextInternal.hpp"
#include "../TRTUtils.hpp"

struct Context {
    using IExecutionContextPtr = std::unique_ptr<nvinfer1::IExecutionContext, TRTDeleter<nvinfer1::IExecutionContext>>;
    IExecutionContextPtr internal_context;

    explicit Context(nvinfer1::IExecutionContext* executionContext) {
        internal_context = IExecutionContextPtr(executionContext);
    }

};

Context_t* create_execution_context(nvinfer1::IExecutionContext* execution_context) {
    return new Context(execution_context);
}

void destroy_excecution_context (Context_t* execution_context) {
    if (execution_context == nullptr)
        return;

    delete execution_context;
}

void context_set_name(Context_t *execution_context, const char * name) {
    if (execution_context == nullptr)
        return;

    execution_context->internal_context->setName(name);
}

const char* context_get_name(Context_t *execution_context) {
    if (execution_context == nullptr)
        return "";

    return execution_context->internal_context->getName();
}

void execute(const Context_t* execution_context, const float* input_data, const size_t input_data_size, const int input_index, float *output_data, const size_t output_data_size, const int output_index) {
    if (execution_context == nullptr)
        return;
    auto& context = execution_context->internal_context;

    cudaError_t rc;

    void* buffers[2];
    rc = cudaMalloc(&buffers[0], input_data_size);
    assert(rc == cudaSuccess);
    rc = cudaMalloc(&buffers[1], output_data_size);
    assert(rc == cudaSuccess);

    rc = cudaMemcpy(buffers[input_index], input_data, input_data_size, cudaMemcpyHostToDevice);
    assert(rc == cudaSuccess);
    context->execute(1, &buffers[input_index]);
    rc = cudaMemcpy(output_data, buffers[output_index], output_data_size, cudaMemcpyDeviceToHost);
    assert(rc == cudaSuccess);

    rc = cudaFree(&buffers[0]);
    assert(rc == cudaSuccess);
    rc = cudaFree(&buffers[1]);
    assert(rc == cudaSuccess);
}