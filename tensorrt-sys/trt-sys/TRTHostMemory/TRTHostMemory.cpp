//
// Created by mason on 1/19/20.
//
#include <cstdlib>
#include <NvInfer.h>

#include "TRTHostMemory.h"

//TODO: Update this struct to use c++ new and delete. See TRTBuilder for example
struct HostMemory {
    void* memory;
};

HostMemory_t* create_host_memory(void* host_memory) {
    HostMemory_t* h;
    h = (typeof(h))malloc(sizeof(h));
    h->memory = host_memory;
    return h;
}

void destroy_host_memory(HostMemory_t* host_memory) {
    if (host_memory == nullptr)
        return;

    auto hostMemory = static_cast<nvinfer1::IHostMemory*>(host_memory->memory);
    hostMemory->destroy();
    free(host_memory);
}

void* host_memory_get_data(HostMemory_t* host_memory) {
    if (host_memory == nullptr)
        return nullptr;

    auto hostMemory = static_cast<nvinfer1::IHostMemory*>(host_memory->memory);
    return hostMemory->data();
}

size_t host_memory_get_size(HostMemory_t* host_memory) {
    if (host_memory == nullptr)
        return -1;

    auto hostMemory = static_cast<nvinfer1::IHostMemory*>(host_memory->memory);
    return hostMemory->size();
}
