//
// Created by mason on 11/27/19.
//
#include "TRTNetworkDefinition.h"

void destroy_network(nvinfer1::INetworkDefinition *network) {
    network->destroy();
}

nvinfer1::ITensor *
network_add_input(nvinfer1::INetworkDefinition *network, const char *name, nvinfer1::DataType type,
                  nvinfer1::Dims dims) {
    return network->addInput(name, type, dims);
}

nvinfer1::ITensor *network_get_input(nvinfer1::INetworkDefinition *network, int32_t idx) {
    return network->getInput(idx);
}

int network_get_nb_layers(nvinfer1::INetworkDefinition *network) {
    return network->getNbLayers();
}

nvinfer1::ILayer *network_get_layer(nvinfer1::INetworkDefinition *network, int index) {
    return network->getLayer(index);
}

nvinfer1::IIdentityLayer *
network_add_identity_layer(nvinfer1::INetworkDefinition *network, nvinfer1::ITensor *inputTensor) {
    return network->addIdentity(*inputTensor);
}

int network_get_nb_inputs(nvinfer1::INetworkDefinition *network) {
    return network->getNbInputs();
}

int network_get_nb_outputs(nvinfer1::INetworkDefinition *network) {
    return network->getNbOutputs();
}

nvinfer1::ITensor *network_get_output(nvinfer1::INetworkDefinition *network, int32_t index) {
    return network->getOutput(index);
}

void network_remove_tensor(nvinfer1::INetworkDefinition *network, nvinfer1::ITensor *tensor) {
    network->removeTensor(*tensor);
}

void network_mark_output(nvinfer1::INetworkDefinition *network, nvinfer1::ITensor *tensor) {
    network->markOutput(*tensor);
}

void network_unmark_output(nvinfer1::INetworkDefinition *network, nvinfer1::ITensor *tensor) {
    network->unmarkOutput(*tensor);
}

nvinfer1::IElementWiseLayer *
network_add_element_wise(nvinfer1::INetworkDefinition *network, nvinfer1::ITensor *input1, nvinfer1::ITensor *input2,
                         nvinfer1::ElementWiseOperation op) {
    return network->addElementWise(*input1, *input2, op);
}

nvinfer1::IGatherLayer *
network_add_gather(nvinfer1::INetworkDefinition *network, nvinfer1::ITensor *data, nvinfer1::ITensor *indices,
                   int32_t axis) {
    return network->addGather(*data, *indices, axis);
}

nvinfer1::IActivationLayer *
network_add_activation(nvinfer1::INetworkDefinition *network, nvinfer1::ITensor *input, nvinfer1::ActivationType type) {
    return network->addActivation(*input, type);
}

nvinfer1::IPoolingLayer *
network_add_pooling(nvinfer1::INetworkDefinition *network, nvinfer1::ITensor *input, nvinfer1::PoolingType poolingType,
                    nvinfer1::DimsHW dims) {
    return network->addPooling(*input, poolingType, dims);
}