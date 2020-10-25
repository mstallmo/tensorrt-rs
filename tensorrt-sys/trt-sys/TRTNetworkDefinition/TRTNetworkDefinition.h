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

nvinfer1::ITensor *network_add_input(Network_t *network, const char *name, DataType_t dataType, Dims_t *dims);

int network_get_nb_layers(Network_t *network);

Layer_t *network_get_layer(Network_t *network, int index);

Layer_t *network_add_identity_layer(Network_t *network, nvinfer1::ITensor *inputTensor);

int network_get_nb_inputs(Network_t *network);

nvinfer1::ITensor *network_get_input(Network_t *network, int32_t idx);

int network_get_nb_outputs(Network_t *network);

nvinfer1::ITensor *network_get_output(Network_t *network, int32_t index);

void network_remove_tensor(Network_t *network, nvinfer1::ITensor *tensor);

void network_mark_output(Network_t *network, nvinfer1::ITensor *tensor);

void network_unmark_output(Network_t *network, nvinfer1::ITensor *tensor);


Layer_t *network_add_element_wise(Network_t *network, nvinfer1::ITensor *input1, nvinfer1::ITensor *input2, ElementWiseOperation_t op);

Layer_t *network_add_gather(Network_t *network, nvinfer1::ITensor *data, nvinfer1::ITensor *indices, int32_t axis);

Layer_t *network_add_activation(Network_t *network, nvinfer1::ITensor *input, ActivationType_t type);

Layer_t *network_add_pooling(Network_t *network, nvinfer1::ITensor *input, PoolingType poolingType, Dims_t *dims);

#ifdef __cplusplus
};
#endif

#endif //LIBTRT_TRTNETWORKDEFINITION_H
