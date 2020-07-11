//
// Created by mason on 11/27/19.
//

#ifndef LIBTRT_TRTBUILDER_H
#define LIBTRT_TRTBUILDER_H

#include "../DLLExport.h"
#include "../TRTLogger/TRTLogger.h"
#include "../TRTNetworkDefinition/TRTNetworkDefinition.h"
#include "../TRTCudaEngine/TRTCudaEngine.h"

#ifdef __cplusplus
extern "C" {
#endif

struct Builder;
typedef struct Builder Builder_t;

DllExport Builder_t *create_infer_builder(Logger_t *logger);
DllExport void destroy_builder(Builder_t* builder);

DllExport Network_t *create_network(Builder_t* builder);

DllExport Engine_t *build_cuda_engine(Builder_t* builder, Network_t* network);

#ifdef __cplusplus
};
#endif

#endif //LIBTRT_TRTBUILDER_H
