//
// Created by mason on 10/7/20.
//

#ifndef LIBTRT_TRTPROFILER_H
#define LIBTRT_TRTPROFILER_H


struct Profiler;
typedef struct Profiler Profiler_t;
class CppProfiler;

CppProfiler* create_profiler(Profiler_t * rust_profiler);
void destroy_profiler(CppProfiler* profiler);


#endif //LIBTRT_TRTPROFILER_H
