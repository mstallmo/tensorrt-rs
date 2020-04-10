//
// Created by mason on 4/9/20.
//

#ifndef LIBTRT_TRTNETWORKDEFINITIONINTERNAL_HPP
#define LIBTRT_TRTNETWORKDEFINITIONINTERNAL_HPP

#include "TRTNetworkDefinition.h"

Network_t* createNetwork(nvinfer1::INetworkDefinition* networkDefinition);
nvinfer1::INetworkDefinition* getNetworkDefinition(const Network_t* network);

#endif //LIBTRT_TRTNETWORKDEFINITIONINTERNAL_HPP
