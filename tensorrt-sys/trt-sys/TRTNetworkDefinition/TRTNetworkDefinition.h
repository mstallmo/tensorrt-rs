//
// Created by mason on 11/27/19.
//

#ifndef LIBTRT_TRTNETWORKDEFINITION_H
#define LIBTRT_TRTNETWORKDEFINITION_H

#include <stdint.h>

#include "../TRTLayer/TRTLayer.h"
#include "../TRTTensor/TRTTensor.h"

#ifdef __cplusplus
extern "C" {
#endif

struct Network;
typedef struct Network Network_t;

void destroy_network(Network_t *network);

Layer_t* network_get_layer(Network_t *network, int index);

Tensor_t *network_get_input(Network_t *network, int32_t idx);

#ifdef __cplusplus
};
#endif

#endif //LIBTRT_TRTNETWORKDEFINITION_H
