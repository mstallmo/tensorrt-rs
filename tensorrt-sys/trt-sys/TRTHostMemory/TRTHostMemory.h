//
// Created by mason on 1/19/20.
//

#ifndef LIBTRT_TRTHOSTMEMORY_H
#define LIBTRT_TRTHOSTMEMORY_H

#ifdef __cplusplus
extern "C" {
#endif

struct HostMemory;
typedef struct HostMemory HostMemory_t;

HostMemory_t* create_host_memory(void* host_memory);

void* host_memory_get_data(HostMemory_t* host_memory);
size_t host_memory_get_size(HostMemory_t* host_memory);

#ifdef __cplusplus
};
#endif




#endif //LIBTRT_TRTHOSTMEMORY_H
