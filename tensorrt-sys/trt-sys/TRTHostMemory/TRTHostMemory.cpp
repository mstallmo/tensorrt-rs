//
// Created by mason on 1/19/20.
//
#include <NvInfer.h>
#include "TRTHostMemory.h"

void destroy_host_memory(nvinfer1::IHostMemory* host_memory) {
    host_memory->destroy();
}

void* host_memory_get_data(nvinfer1::IHostMemory* host_memory) {
    return host_memory->data();
}

size_t host_memory_get_size(nvinfer1::IHostMemory* host_memory) {
    return host_memory->size();
}
