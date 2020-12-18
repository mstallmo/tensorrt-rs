//
// Created by mason on 10/7/20.
//

#ifndef LIBTRT_TRTPROFILERINTERNAL_HPP
#define LIBTRT_TRTPROFILERINTERNAL_HPP

#include "TRTProfiler.h"
#include <NvInfer.h>

struct Profiler {
    void (*reportLayerTime)(void *context, const char *layerName, float ms);

    void (*destroy)(void *self, void *context);

    void *context;
};

class CppProfiler : public nvinfer1::IProfiler {
public:
    explicit CppProfiler(Profiler_t *_profiler) : profiler(_profiler) {}
    ~CppProfiler();

    void reportLayerTime(const char *layerName, float ms) override;

    Profiler_t *getInternalProfiler() {
        return profiler;
    }

private:
    Profiler_t *profiler;
};

#endif //LIBTRT_TRTPROFILERINTERNAL_HPP
