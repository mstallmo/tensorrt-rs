//
// Created by mason on 1/19/20.
//

#ifndef LIBTRT_TRTHOSTMEMORY_H
#define LIBTRT_TRTHOSTMEMORY_H

#include <NvInfer.h>

void destroy_host_memory(nvinfer1::IHostMemory* host_memory);

void* host_memory_get_data(nvinfer1::IHostMemory* host_memory);
size_t host_memory_get_size(nvinfer1::IHostMemory* host_memory);


#endif //LIBTRT_TRTHOSTMEMORY_H
