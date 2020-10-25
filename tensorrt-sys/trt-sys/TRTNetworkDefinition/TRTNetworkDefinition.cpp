//
// Created by mason on 11/27/19.
//
#include <NvInfer.h>

#include "TRTNetworkDefinitionInternal.hpp"
#include "../TRTLayer/TRTLayerInternal.hpp"
#include "../TRTDims/TRTDimsInternal.hpp"

void destroy_network(Network_t *network) {
    if (network == nullptr)
        return;

    delete network;
}

nvinfer1::INetworkDefinition &Network::getNetworkDefinition() const {
    return *this->internal_network;
}

nvinfer1::ITensor *network_add_input(Network_t *network, const char *name, DataType_t type, Dims_t *dims) {
    return network->internal_network->addInput(name, static_cast<nvinfer1::DataType>(type), dims_get(dims));
}

int network_get_nb_layers(Network_t *network) {
    return network->internal_network->getNbLayers();
}

Layer_t *network_get_layer(Network_t *network, int index) {
    if (network == nullptr)
        return nullptr;

    return new Layer(network->internal_network->getLayer(index));
}

Layer_t *network_add_identity_layer(Network_t *network, nvinfer1::ITensor *inputTensor) {
    return new Layer(network->internal_network->addIdentity(*inputTensor));
}

int network_get_nb_inputs(Network_t *network) {
    return network->internal_network->getNbInputs();
}

nvinfer1::ITensor *network_get_input(Network_t *network, int32_t idx) {
    return network->internal_network->getInput(idx);
}

int network_get_nb_outputs(Network_t *network) {
    return network->internal_network->getNbOutputs();
}

nvinfer1::ITensor *network_get_output(Network_t *network, int32_t index) {
    return network->internal_network->getOutput(index);
}

void network_remove_tensor(Network_t *network, nvinfer1::ITensor *tensor) {
    network->internal_network->removeTensor(*tensor);
}

void network_mark_output(Network_t *network, nvinfer1::ITensor *tensor) {
    network->internal_network->markOutput(*tensor);
}

void network_unmark_output(Network_t *network, nvinfer1::ITensor *tensor) {
    network->internal_network->unmarkOutput(*tensor);
}

Layer_t *network_add_element_wise(Network_t *network, nvinfer1::ITensor *input1, nvinfer1::ITensor *input2, ElementWiseOperation_t op) {
    return new Layer(network->internal_network->addElementWise(*input1, *input2,
                                                               static_cast<nvinfer1::ElementWiseOperation>(op)));
}

Layer_t *network_add_gather(Network_t *network, nvinfer1::ITensor *data, nvinfer1::ITensor *indices, int32_t axis) {
    return new Layer(network->internal_network->addGather(*data, *indices, axis));
}

Layer_t *network_add_activation(Network_t *network, nvinfer1::ITensor *input, ActivationType_t type) {
    return new Layer(network->internal_network->addActivation(*input,
                                                              static_cast<nvinfer1::ActivationType>(type)));
}

Layer_t *network_add_pooling(Network_t *network, nvinfer1::ITensor *input, PoolingType poolingType, Dims_t *dims) {
    return new Layer(network->internal_network->addPooling(*input,
                                                           static_cast<nvinfer1::PoolingType>(poolingType),
                                                           dimsHW_get(dims)));
}