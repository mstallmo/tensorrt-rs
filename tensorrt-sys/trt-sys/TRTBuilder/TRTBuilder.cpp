//
// Created by mason on 11/27/19.
//
#include <cstdlib>
#include <NvInfer.h>

#include "TRTBuilder.h"
#include "../TRTNetworkDefinition/TRTNetworkDefinitionInternal.hpp"

#define MAX_WORKSPACE (1 << 30)

Builder_t* create_infer_builder(Logger_t* logger)
{
    Builder_t* b;
    nvinfer1::IBuilder* builder;
    auto l = static_cast<nvinfer1::ILogger*>(logger->internal_logger);

    builder = nvinfer1::createInferBuilder(*l);
    builder->setMaxWorkspaceSize(MAX_WORKSPACE);
    builder->setMaxBatchSize(1);
    b = (typeof(b))malloc(sizeof(*b));
    b->internal_builder = builder;

    return b;
}

void destroy_builder(Builder_t* builder)
{
    if(builder == nullptr)
        return;

    auto b = static_cast<nvinfer1::IBuilder*>(builder->internal_builder);

    b->destroy();
    free(builder);
}

Network_t* create_network(Builder_t *builder)
{
    auto b = static_cast<nvinfer1::IBuilder*>(builder->internal_builder);
    return createNetwork(b->createNetwork());
}

Engine_t *build_cuda_engine(Builder_t *builder, Network_t *network)
{
    auto b = static_cast<nvinfer1::IBuilder*>(builder->internal_builder);

    auto engine = b->buildCudaEngine(*getNetworkDefinition(network));
    return create_engine(engine);
}