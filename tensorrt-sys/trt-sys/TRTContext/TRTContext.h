#pragma clang diagnostic push
#pragma ide diagnostic ignored "modernize-deprecated-headers"
//
// Created by mason on 9/17/19.
//

#ifndef LIBTRT_TRTCONTEXT_H
#define LIBTRT_TRTCONTEXT_H

#include <stddef.h>
#include <stdbool.h>

#include <NvInfer.h>
#include "../TRTProfiler/TRTProfiler.h"


void destroy_excecution_context(nvinfer1::IExecutionContext* execution_context);

void context_set_debug_sync(nvinfer1::IExecutionContext* execution_context, bool sync);
bool context_get_debug_sync(nvinfer1::IExecutionContext* execution_context);

void context_set_name(nvinfer1::IExecutionContext* execution_context, const char *name);
const char* context_get_name(nvinfer1::IExecutionContext *execution_context);

//void context_set_profiler(Context_t execution_context, Profiler_t* profiler);
//Profiler_t* context_get_profiler(Context_t *execution_context);

void execute(nvinfer1::IExecutionContext* execution_context, const void** binding_data, const int num_bindings, const size_t* data_sizes);


#endif //LIBTRT_TRTCONTEXT_H

#pragma clang diagnostic pop