//
// Created by mason on 11/27/19.
//
#include <NvInfer.h>

#include "TRTNetworkDefinitionInternal.hpp"
#include "../TRTDims/TRTDimsInternal.hpp"

void destroy_network(Network_t *network) {
    if (network == nullptr)
        return;

    delete network;
}

nvinfer1::INetworkDefinition& Network::getNetworkDefinition() const {
   return *this->internal_network;
}

Tensor_t *network_get_input(Network_t *network, int32_t idx) {
    if (network == nullptr) {
        return nullptr;
	}

	return new Tensor(network->internal_network->getInput(idx));
}

void tensor_set_dimensions(Tensor_t *tensor, Dims_t *dimensions) {
    if (tensor == nullptr) {
        return;
	}

	tensor->internal_tensor->setDimensions(dims_get(dimensions));
}
