//
// Created by mason on 11/27/19.
//
#include <NvInfer.h>

#include "TRTNetworkDefinitionInternal.hpp"
#include "../TRTLayer/TRTLayerInternal.hpp"
#include "../TRTTensor/TRTTensorInternal.hpp"

void destroy_network(Network_t *network) {
    if (network == nullptr)
        return;

    delete network;
}

nvinfer1::INetworkDefinition& Network::getNetworkDefinition() const {
   return *this->internal_network;
}

Tensor_t *network_get_input(Network_t *network, int32_t idx) {
    return new Tensor(network->internal_network->getInput(idx));
}

Layer_t* network_get_layer(Network_t *network, int index) {
    if (network == nullptr)
        return nullptr;

    return new Layer(network->internal_network->getLayer(index));
}

