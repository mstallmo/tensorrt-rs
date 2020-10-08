//
// Created by mason on 11/27/19.
//

#ifndef LIBTRT_TRTNETWORKDEFINITION_H
#define LIBTRT_TRTNETWORKDEFINITION_H

#ifdef __cplusplus
extern "C" {
#endif

#include <stdint.h>

#include "../TRTDims/TRTDims.h"
#include "../TRTLayer/TRTLayer.h"

struct Network;
typedef struct Network Network_t;

struct Tensor;
typedef struct Tensor Tensor_t;

void destroy_network(Network_t *network);

Layer_t* network_get_layer(Network_t *network, int index);

Tensor_t *network_get_input(Network_t *network, int32_t idx);

void tensor_set_dimensions(Tensor_t *tensor, Dims_t *dimensions); // only valid for input tensors

#ifdef __cplusplus
};
#endif

#endif //LIBTRT_TRTNETWORKDEFINITION_H
