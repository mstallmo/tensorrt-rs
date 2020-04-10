//
// Created by mason on 11/27/19.
//
#include <cstdlib>
#include <new>
#include <NvInfer.h>

#include "TRTNetworkDefinitionInternal.hpp"

struct Network {
    nvinfer1::INetworkDefinition* internal_network;

    explicit Network(nvinfer1::INetworkDefinition* networkDefinition) : internal_network(networkDefinition) {};
    ~Network() {
        internal_network->destroy();
    }
};

Network_t* createNetwork(nvinfer1::INetworkDefinition* networkDefinition) {
    return new Network(networkDefinition);
}

void destroy_network(Network_t *network) {
    if (network == nullptr)
        return;

    delete network;
}

nvinfer1::INetworkDefinition *getNetworkDefinition(const Network_t *network) {
   if (network == nullptr)
       return nullptr;

   return network->internal_network;
}
