//
// Created by mason on 11/27/19.
//

#ifndef LIBTRT_TRTBUILDER_H
#define LIBTRT_TRTBUILDER_H

#include "../TRTLogger/TRTLogger.h"
#include "../TRTNetworkDefinition/TRTNetworkDefinition.h"
#include "../TRTCudaEngine/TRTCudaEngine.h"

struct Builder {
    void *internal_builder;
};
typedef struct Builder Builder_t;

#ifdef __cplusplus
extern "C" {
#endif

Builder_t *create_infer_builder(Logger_t *logger);
void destroy_builder(Builder_t* builder);

Network_t *create_network(Builder_t* builder);

Engine_t *build_cuda_engine(Builder_t* builder, Network_t* network);

#ifdef __cplusplus
};
#endif

#endif //LIBTRT_TRTBUILDER_H
