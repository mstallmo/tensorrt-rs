//
// Created by mason on 4/9/20.
//

#ifndef LIBTRT_TRTNETWORKDEFINITIONINTERNAL_HPP
#define LIBTRT_TRTNETWORKDEFINITIONINTERNAL_HPP

#include <memory>

#include <NvInfer.h>

#include "TRTNetworkDefinition.h"
#include "../TRTUtils.hpp"

struct Network {
    using INetworkDefinitionPtr = std::unique_ptr<nvinfer1::INetworkDefinition, TRTDeleter<nvinfer1::INetworkDefinition>>;
    INetworkDefinitionPtr internal_network;

    explicit Network(nvinfer1::INetworkDefinition* networkDefinition) : internal_network(networkDefinition) {};
    [[nodiscard]] nvinfer1::INetworkDefinition& getNetworkDefinition() const;
};

struct Tensor {
    nvinfer1::ITensor *internal_tensor;

    explicit Tensor(nvinfer1::ITensor *tensor) : internal_tensor(tensor) {};
};


#endif //LIBTRT_TRTNETWORKDEFINITIONINTERNAL_HPP
