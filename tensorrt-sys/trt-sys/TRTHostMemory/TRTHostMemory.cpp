//
// Created by mason on 1/19/20.
//
#include <memory>
#include <NvInfer.h>

#include "../TRTUtils.hpp"
#include "TRTHostMemoryInternal.hpp"

struct HostMemory {
    using IHostMemoryPtr = std::unique_ptr<nvinfer1::IHostMemory, TRTDeleter<nvinfer1::IHostMemory>>;
    IHostMemoryPtr internal_host_memory;

    explicit HostMemory(nvinfer1::IHostMemory* hostMemory) {
        internal_host_memory = IHostMemoryPtr(hostMemory);
    }
};

HostMemory_t* create_host_memory(nvinfer1::IHostMemory* host_memory) {
    return new HostMemory(host_memory);
}

void destroy_host_memory(HostMemory_t* host_memory) {
    if (host_memory == nullptr)
        return;

    delete host_memory;
}

void* host_memory_get_data(HostMemory_t* host_memory) {
    if (host_memory == nullptr)
        return nullptr;

    return host_memory->internal_host_memory->data();
}

size_t host_memory_get_size(HostMemory_t* host_memory) {
    if (host_memory == nullptr)
        return -1;

    return host_memory->internal_host_memory->size();
}
