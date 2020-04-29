//
// Created by mason on 4/28/20.
//

#ifndef LIBTRT_TRTHOSTMEMORYINTERNAL_HPP
#define LIBTRT_TRTHOSTMEMORYINTERNAL_HPP

#include <NvInfer.h>

#include "TRTHostMemory.h"

HostMemory_t* create_host_memory(nvinfer1::IHostMemory* host_memory);

#endif //LIBTRT_TRTHOSTMEMORYINTERNAL_HPP
