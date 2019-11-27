//
// Created by mason on 8/25/19.
//
#include <cstdlib>
#include "NvInfer.h"
#include "TRTRuntime.h"

struct Runtime {
    void* internal_runtime;
};

Runtime_t* create_infer_runtime(Logger_t* logger) {
    Runtime_t* r;
    nvinfer1::IRuntime* runtime;
    auto l = static_cast<nvinfer1::ILogger* >(logger->internal_logger);

    r = (typeof(r))malloc(sizeof(*r));
    runtime = nvinfer1::createInferRuntime(*l);
    r->internal_runtime = runtime;

    return r;
}

Engine_t* deserialize_cuda_engine(Runtime_t* runtime, const void* blob, unsigned long long size) {
    auto mRuntime = static_cast<nvinfer1::IRuntime* >(runtime->internal_runtime);
    nvinfer1::ICudaEngine* engine = mRuntime->deserializeCudaEngine(blob, size, nullptr);

    return create_engine(engine);
}

void destroy_infer_runtime(Runtime_t* runtime) {
    if (runtime == nullptr)
        return;

    auto rt = static_cast<nvinfer1::IRuntime* >(runtime->internal_runtime);
    rt->destroy();
    free(runtime);
}
