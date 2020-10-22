//
// Created by mason on 8/25/19.
//
#include "NvInfer.h"
#include "TRTRuntime.h"

#include "../TRTLogger/TRTLoggerInternal.hpp"
#include "../TRTUtils.hpp"

struct Runtime {
    using IRuntimePtr = std::unique_ptr<nvinfer1::IRuntime, TRTDeleter<nvinfer1::IRuntime>>;
    IRuntimePtr internal_runtime;

    explicit Runtime(nvinfer1::ILogger& logger) {
        internal_runtime = IRuntimePtr(nvinfer1::createInferRuntime(logger));
    }
};

Runtime_t* create_infer_runtime(Logger_t* logger) {
    return new Runtime(logger->getLogger());
}

void destroy_infer_runtime(Runtime_t* runtime) {
    if (runtime == nullptr)
        return;

    delete runtime;
}

nvinfer1::ICudaEngine *deserialize_cuda_engine(Runtime_t *runtime, const void *blob, unsigned long long size) {
    return runtime->internal_runtime->deserializeCudaEngine(blob, size, nullptr);
}

int runtime_get_nb_dla_cores(Runtime_t *runtime) {
    return runtime->internal_runtime->getNbDLACores();
}

int runtime_get_dla_core(Runtime_t *runtime) {
    return runtime->internal_runtime->getDLACore();
}

void runtime_set_dla_core(Runtime_t *runtime, int dla_core) {
    runtime->internal_runtime->setDLACore(dla_core);
}
