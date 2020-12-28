//
// Created by mason on 12/16/20.
//

#include "TRTProfilerInternal.hpp"
#include <iostream>

CppProfiler* create_profiler(Profiler_t * rust_profiler) {
    return new CppProfiler(rust_profiler);
}

void destroy_profiler(CppProfiler* profiler) {
   delete profiler;
}

CppProfiler::~CppProfiler() {
    (*profiler->destroy)(profiler, profiler->context);
}

void CppProfiler::reportLayerTime(const char *layerName, float ms) {
    (*profiler->reportLayerTime)(profiler->context, layerName, ms);
}