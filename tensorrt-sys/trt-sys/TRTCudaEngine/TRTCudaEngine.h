//
// Created by mason on 8/26/19.
//

#ifndef LIBTRT_TRTCUDAENGINE_H
#define LIBTRT_TRTCUDAENGINE_H

#ifdef __cplusplus
extern "C" {
#endif

#include "../TRTContext/TRTContext.h"
#include "../TRTHostMemory/TRTHostMemory.h"
#include "../TRTDims/TRTDims.h"

struct Engine;
typedef struct Engine Engine_t;

void destroy_cuda_engine(Engine_t* engine);

Context_t* engine_create_execution_context(Engine_t* engine);

int get_nb_bindings(Engine_t* engine);
const char* get_binding_name(Engine_t* engine, int binding_index);
int get_binding_index(Engine_t *engine, const char* op_name);
Dims_t* get_binding_dimensions(Engine_t *engine, int binding_index);

HostMemory_t* engine_serialize(Engine_t* engine);

#ifdef __cplusplus
};
#endif

#endif //LIBTRT_TRTCUDAENGINE_H
