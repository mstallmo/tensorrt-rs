//
// Created by mason on 1/19/20.
//
#include <cstdlib>
#include <NvInfer.h>

#include "TRTHostMemory.h"

struct HostMemory {
    void* memory;
};

HostMemory_t* create_host_memory(void* host_memory) {
    HostMemory_t* h;
    h = (typeof(h))malloc(sizeof(h));
    h->memory = host_memory;
    return h;
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
