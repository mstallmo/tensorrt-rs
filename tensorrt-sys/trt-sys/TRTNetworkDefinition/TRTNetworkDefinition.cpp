//
// Created by mason on 11/27/19.
//
#include <NvInfer.h>

#include "TRTNetworkDefinitionInternal.hpp"

void destroy_network(Network_t *network) {
    if (network == nullptr)
        return;

    delete network;
}

nvinfer1::INetworkDefinition& Network::getNetworkDefinition() const {
   return *this->internal_network;
}
