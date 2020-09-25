//
// Created by mason on 11/27/19.
//

#ifndef LIBTRT_TRTBUILDER_H
#define LIBTRT_TRTBUILDER_H

#include "../TRTLogger/TRTLogger.h"
#include "../TRTNetworkDefinition/TRTNetworkDefinition.h"
#include "../TRTCudaEngine/TRTCudaEngine.h"

#include <stddef.h>
#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

struct Builder;
typedef struct Builder Builder_t;

Builder_t *create_infer_builder(Logger_t *logger);
void destroy_builder(Builder_t* builder);

void builder_set_max_batch_size(Builder_t* builder, int32_t batch_size);
int32_t builder_get_max_batch_size(Builder_t* builder);

void builder_set_max_workspace_size(Builder_t* builder, size_t batch_size);
size_t builder_get_max_workspace_size(Builder_t* builder);

Network_t *create_network(Builder_t* builder);
Network_t *create_network_v2(Builder_t* builder, uint32_t flags);

Engine_t *build_cuda_engine(Builder_t* builder, Network_t* network);

#ifdef __cplusplus
};
#endif

#endif //LIBTRT_TRTBUILDER_H
