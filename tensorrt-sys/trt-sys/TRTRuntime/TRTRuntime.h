//
// Created by mason on 8/25/19.
//

#ifndef TENSRORT_SYS_TRTRUNTIME_H
#define TENSRORT_SYS_TRTRUNTIME_H

#include <NvInfer.h>
#include "../TRTLogger/TRTLogger.h"
#include "../TRTCudaEngine/TRTCudaEngine.h"

#ifdef __cplusplus
extern "C" {
#endif

struct Runtime;
typedef struct Runtime Runtime_t;

Runtime_t *create_infer_runtime(Logger_t *logger);
void destroy_infer_runtime(Runtime_t *runtime);

int runtime_get_nb_dla_cores(Runtime_t *runtime);
int runtime_get_dla_core(Runtime_t *runtime);
void runtime_set_dla_core(Runtime_t *runtime, int dla_core);

#ifdef __cplusplus
};
#endif

nvinfer1::ICudaEngine *deserialize_cuda_engine(Runtime_t *runtime, const void *blob, unsigned long long size);

#endif //TENSRORT_SYS_TRTRUNTIME_H
