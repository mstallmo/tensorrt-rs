//
// Created by mason on 8/25/19.
//
#include "TRTRuntime.h"
#include "../TRTLogger/TRTLoggerInternal.hpp"

nvinfer1::IRuntime *create_infer_runtime(Logger_t *logger) {
    return nvinfer1::createInferRuntime(logger->getLogger());
}

void destroy_infer_runtime(nvinfer1::IRuntime *runtime) {
    runtime->destroy();
}

nvinfer1::ICudaEngine *deserialize_cuda_engine(nvinfer1::IRuntime *runtime, const void *blob, unsigned long long size) {
    return runtime->deserializeCudaEngine(blob, size, nullptr);
}

int runtime_get_nb_dla_cores(nvinfer1::IRuntime *runtime) {
    return runtime->getNbDLACores();
}

int runtime_get_dla_core(nvinfer1::IRuntime *runtime) {
    return runtime->getDLACore();
}

void runtime_set_dla_core(nvinfer1::IRuntime *runtime, int dla_core) {
    runtime->setDLACore(dla_core);
}
