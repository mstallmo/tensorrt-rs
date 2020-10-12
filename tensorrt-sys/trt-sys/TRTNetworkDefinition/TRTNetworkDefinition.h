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

Tensor_t* network_add_input(Network_t *network, const char* name, DataType_t dataType, Dims_t *dims);

int network_get_nb_layers(Network_t *network);

Layer_t* network_get_layer(Network_t *network, int index);

Layer_t* network_add_identity_layer(Network_t *network, Tensor_t* inputTensor);

int network_get_nb_inputs(Network_t *network);

Tensor_t *network_get_input(Network_t *network, int32_t idx);

int network_get_nb_outputs(Network_t *network);

Tensor_t *network_get_output(Network_t *network, int32_t index);

void network_remove_tensor(Network_t *network, Tensor_t *tensor);

void network_mark_output(Network_t *network, Tensor_t *tensor);

void network_unmark_output(Network_t *network, Tensor_t *tensor);


#ifdef __cplusplus
};
#endif

#endif //LIBTRT_TRTNETWORKDEFINITION_H
