//
// Created by mason on 10/7/20.
//

#ifndef LIBTRT_TRTPROFILERINTERNAL_HPP
#define LIBTRT_TRTPROFILERINTERNAL_HPP

#include "TRTProfiler.h"
#include <NvInfer.h>

struct Profiler {
    void (*reportLayerTime)(void *context, const char *layerName, float ms);
    void (*destroy)(void *self, void* context);
    void* context;
};

class ConcreteProfiler : public nvinfer1::IProfiler {
public:
    explicit ConcreteProfiler(Profiler_t *_profiler) : profiler(_profiler) {}

    void reportLayerTime(const char* layerName, float ms) override {
        (*profiler->reportLayerTime)(profiler->context, layerName, ms);
    }

    void destroy() {
        (*profiler->destroy)(profiler, profiler->context);
    }

    Profiler_t* getInternalProfiler() {
        return profiler;
    }

private:
    Profiler_t *profiler;
};

#endif //LIBTRT_TRTPROFILERINTERNAL_HPP
