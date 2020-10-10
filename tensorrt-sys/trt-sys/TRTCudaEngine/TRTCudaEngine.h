//
// Created by mason on 8/26/19.
//

#ifndef LIBTRT_TRTCUDAENGINE_H
#define LIBTRT_TRTCUDAENGINE_H

#include "../TRTContext/TRTContext.h"
#include "../TRTHostMemory/TRTHostMemory.h"
#include "../TRTDims/TRTDims.h"

#ifdef __cplusplus
extern "C" {
#endif

enum DataType {
    kFLOAT = 0,
    kHALF = 1,
    kINT8 = 2,
    kINT32 = 3,
};
typedef enum DataType DataType_t;

enum TensorLocation {
    kDEVICE = 0,
    kHOST = 1,
};
typedef enum TensorLocation TensorLocation_t;

struct Engine;
typedef struct Engine Engine_t;

void engine_destroy(Engine_t* engine);

Context_t* engine_create_execution_context(Engine_t* engine);
Context_t* engine_create_execution_context_without_device_memory(Engine_t *engine);

int engine_get_nb_bindings(Engine_t* engine);

int engine_get_binding_index(Engine_t *engine, const char* op_name);

const char* engine_get_binding_name(Engine_t* engine, int binding_index);

bool engine_binding_is_input(Engine_t *engine, int binding_index);

Dims_t* engine_get_binding_dimensions(Engine_t *engine, int binding_index);

DataType_t engine_get_binding_data_type(Engine_t *engine, int binding_index);

int engine_get_max_batch_size(Engine_t *engine);

int engine_get_nb_layers(Engine_t *engine);

size_t engine_get_workspace_size(Engine_t *engine);

HostMemory_t* engine_serialize(Engine_t* engine);

TensorLocation_t engine_get_location(Engine_t *engine, int binding_index);

size_t engine_get_device_memory_size(Engine_t *engine);

bool engine_is_refittable(Engine_t *engine);

#ifdef __cplusplus
};
#endif

#endif //LIBTRT_TRTCUDAENGINE_H
