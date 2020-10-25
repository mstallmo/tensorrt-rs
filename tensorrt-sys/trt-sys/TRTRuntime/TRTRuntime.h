//
// Created by mason on 8/25/19.
//

#ifndef TENSRORT_SYS_TRTRUNTIME_H
#define TENSRORT_SYS_TRTRUNTIME_H

#include <NvInfer.h>
#include "../TRTLogger/TRTLogger.h"
#include "../TRTCudaEngine/TRTCudaEngine.h"

nvinfer1::IRuntime *create_infer_runtime(Logger_t *logger);
void destroy_infer_runtime(nvinfer1::IRuntime *runtime);
nvinfer1::ICudaEngine *deserialize_cuda_engine(nvinfer1::IRuntime *runtime, const void *blob, unsigned long long size);
int runtime_get_nb_dla_cores(nvinfer1::IRuntime *runtime);
int runtime_get_dla_core(nvinfer1::IRuntime *runtime);
void runtime_set_dla_core(nvinfer1::IRuntime *runtime, int dla_core);

#endif //TENSRORT_SYS_TRTRUNTIME_H
