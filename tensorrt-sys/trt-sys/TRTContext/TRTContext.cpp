//
// Created by mason on 9/17/19.
//
#include <cstdlib>
#include <cuda_runtime.h>
#include "NvInfer.h"
#include "TRTContext.h"

struct Context {
    void* internal_context;
};

Context_t* create_execution_context(void* execution_context) {
    Context_t* c;
    c = (typeof(c))malloc(sizeof(c));
    c->internal_context = execution_context;
    return c;
}

void destroy_excecution_context (Context_t* execution_context) {
    if (execution_context == nullptr)
        return;

    auto executionContext = static_cast<nvinfer1::IExecutionContext *>(execution_context->internal_context);
    executionContext->destroy();
    free(execution_context);
}

void context_set_name(Context_t *execution_context, const char * name) {
    if (execution_context == nullptr)
        return;

    auto executionContext = static_cast<nvinfer1::IExecutionContext *>(execution_context->internal_context);
    executionContext->setName(name);
}

const char* context_get_name(Context_t *execution_context) {
    if (execution_context == nullptr)
        return "";

    auto executionContext = static_cast<nvinfer1::IExecutionContext *>(execution_context->internal_context);
    return executionContext->getName();
}

 void execute(Context_t* execution_context, const float* input_data, const size_t input_data_size, const int input_index, float *output_data, const size_t output_data_size, const int output_index) {
    if (execution_context == nullptr)
        return;
    auto context = static_cast<nvinfer1::IExecutionContext *>(execution_context->internal_context);

    void* buffers[2];
    cudaMalloc(&buffers[0], input_data_size);
    cudaMalloc(&buffers[1], output_data_size);

    cudaMemcpy(buffers[input_index], input_data, input_data_size, cudaMemcpyHostToDevice);
    context->execute(1, &buffers[input_index]);
    cudaMemcpy(output_data, buffers[output_index], output_data_size, cudaMemcpyDeviceToHost);

    cudaFree(&buffers[0]);
    cudaFree(&buffers[1]);
}