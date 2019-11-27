//
// Created by mason on 11/27/19.
//
#include <cstdlib>
#include <NvInfer.h>

#include "TRTNetworkDefinition.h"

void destroy_network(Network_t *network)
{
    if(network == nullptr)
        return;
    auto n = static_cast<nvinfer1::INetworkDefinition*>(network->internal_network);
    n->destroy();
    free(network);
}
