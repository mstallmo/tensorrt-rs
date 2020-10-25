//
// Created by mason on 11/27/19.
//

#ifndef LIBTRT_TRTNETWORKDEFINITION_H
#define LIBTRT_TRTNETWORKDEFINITION_H

#include <stdint.h>
#include <NvInfer.h>

#include "../TRTLayer/TRTLayer.h"
#include "../TRTTensor/TRTTensor.h"

void destroy_network(nvinfer1::INetworkDefinition *network);
nvinfer1::ITensor *network_add_input(nvinfer1::INetworkDefinition *network, const char *name, DataType_t dataType, Dims_t *dims);
nvinfer1::ITensor *network_get_input(nvinfer1::INetworkDefinition *network, int32_t idx);
int network_get_nb_layers(nvinfer1::INetworkDefinition *network);
nvinfer1::ILayer *network_get_layer(nvinfer1::INetworkDefinition *network, int index);
int network_get_nb_inputs(nvinfer1::INetworkDefinition *network);
int network_get_nb_outputs(nvinfer1::INetworkDefinition *network);
nvinfer1::ITensor *network_get_output(nvinfer1::INetworkDefinition *network, int32_t index);
void network_remove_tensor(nvinfer1::INetworkDefinition *network, nvinfer1::ITensor *tensor);
void network_mark_output(nvinfer1::INetworkDefinition *network, nvinfer1::ITensor *tensor);
void network_unmark_output(nvinfer1::INetworkDefinition *network, nvinfer1::ITensor *tensor);
nvinfer1::IIdentityLayer *network_add_identity_layer(nvinfer1::INetworkDefinition *network, nvinfer1::ITensor *inputTensor);
nvinfer1::IElementWiseLayer *network_add_element_wise(nvinfer1::INetworkDefinition *network, nvinfer1::ITensor *input1, nvinfer1::ITensor *input2, ElementWiseOperation_t op);
nvinfer1::IGatherLayer *network_add_gather(nvinfer1::INetworkDefinition *network, nvinfer1::ITensor *data, nvinfer1::ITensor *indices, int32_t axis);
nvinfer1::IActivationLayer *network_add_activation(nvinfer1::INetworkDefinition *network, nvinfer1::ITensor *input, ActivationType_t type);
nvinfer1::IPoolingLayer *network_add_pooling(nvinfer1::INetworkDefinition *network, nvinfer1::ITensor *input, PoolingType poolingType, Dims_t *dims);

#endif //LIBTRT_TRTNETWORKDEFINITION_H
