//
// Created by mason on 11/27/19.
//
#include <NvInfer.h>

#include "TRTNetworkDefinitionInternal.hpp"
#include "../TRTLayer/TRTLayerInternal.hpp"
#include "../TRTTensor/TRTTensorInternal.hpp"
#include "../TRTDims/TRTDimsInternal.hpp"

void destroy_network(Network_t *network) {
    if (network == nullptr)
        return;

    delete network;
}

nvinfer1::INetworkDefinition &Network::getNetworkDefinition() const {
    return *this->internal_network;
}

Tensor_t *network_add_input(Network_t *network, const char *name, DataType_t type, Dims_t *dims) {
    return new Tensor(network->internal_network->addInput(name, static_cast<nvinfer1::DataType>(type), dims_get(dims)));
}

int network_get_nb_layers(Network_t *network) {
    return network->internal_network->getNbLayers();
}

Layer_t *network_get_layer(Network_t *network, int index) {
    if (network == nullptr)
        return nullptr;

    return new Layer(network->internal_network->getLayer(index));
}

Layer_t *network_add_identity_layer(Network_t *network, Tensor_t *inputTensor) {
    return new Layer(network->internal_network->addIdentity(*inputTensor->internal_tensor));
}

int network_get_nb_inputs(Network_t *network) {
    return network->internal_network->getNbInputs();
}

Tensor_t *network_get_input(Network_t *network, int32_t idx) {
    return new Tensor(network->internal_network->getInput(idx));
}

int network_get_nb_outputs(Network_t *network) {
    return network->internal_network->getNbOutputs();
}

Tensor_t *network_get_output(Network_t *network, int32_t index) {
    return new Tensor(network->internal_network->getOutput(index));
}

void network_remove_tensor(Network_t *network, Tensor_t *tensor) {
    network->internal_network->removeTensor(*tensor->internal_tensor);
}

void network_mark_output(Network_t *network, Tensor_t *tensor) {
    network->internal_network->markOutput(*tensor->internal_tensor);
}

void network_unmark_output(Network_t *network, Tensor_t *tensor) {
    network->internal_network->unmarkOutput(*tensor->internal_tensor);
}

Layer_t *network_add_element_wise(Network_t *network, Tensor_t *input1, Tensor_t *input2, ElementWiseOperation_t op) {
    return new Layer(network->internal_network->addElementWise(*input1->internal_tensor, *input2->internal_tensor,
                                                               static_cast<nvinfer1::ElementWiseOperation>(op)));
}

Layer_t *network_add_gather(Network_t *network, Tensor_t *data, Tensor_t *indices, int32_t axis) {
    return new Layer(network->internal_network->addGather(*data->internal_tensor, *indices->internal_tensor, axis));
}
