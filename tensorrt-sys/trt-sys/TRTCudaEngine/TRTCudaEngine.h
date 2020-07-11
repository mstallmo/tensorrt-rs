//
// Created by mason on 8/26/19.
//

#ifndef LIBTRT_TRTCUDAENGINE_H
#define LIBTRT_TRTCUDAENGINE_H

#ifdef __cplusplus
extern "C" {
#endif

#include "../DLLExport.h"
#include "../TRTContext/TRTContext.h"
#include "../TRTHostMemory/TRTHostMemory.h"

struct Engine;
typedef struct Engine Engine_t;

DllExport void destroy_cuda_engine(Engine_t* engine);

DllExport Context_t* engine_create_execution_context(Engine_t* engine);

DllExport int get_nb_bindings(Engine_t* engine);
DllExport const char* get_binding_name(Engine_t* engine, int binding_index);
DllExport int get_binding_index(Engine_t *engine, const char* op_name);

DllExport HostMemory_t* engine_serialize(Engine_t* engine);

#ifdef __cplusplus
};
#endif

#endif //LIBTRT_TRTCUDAENGINE_H
