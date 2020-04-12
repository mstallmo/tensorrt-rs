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

Engine_t* deserialize_cuda_engine(Runtime_t* runtime, const void* blob, unsigned long long size) {
    nvinfer1::ICudaEngine* engine = runtime->internal_runtime->deserializeCudaEngine(blob, size, nullptr);

    return create_engine(engine);
}

void destroy_infer_runtime(Runtime_t* runtime) {
    if (runtime == nullptr)
        return;

    delete runtime;
}
