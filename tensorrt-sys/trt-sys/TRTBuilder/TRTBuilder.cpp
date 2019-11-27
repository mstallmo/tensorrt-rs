//
// Created by mason on 11/27/19.
//
#include <cstdlib>
#include <NvInfer.h>

#include "TRTBuilder.h"

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
    Network_t* n;
    nvinfer1::INetworkDefinition* networkDefinition;
    auto b = static_cast<nvinfer1::IBuilder*>(builder->internal_builder);

    networkDefinition = b->createNetwork();
    n = (typeof(n))malloc(sizeof(n));
    n->internal_network = networkDefinition;
    return n;
}

Engine_t *build_cuda_engine(Builder_t *builder, Network_t *network)
{
    auto n = static_cast<nvinfer1::INetworkDefinition*>(network->internal_network);
    auto b = static_cast<nvinfer1::IBuilder*>(builder->internal_builder);

    auto engine = b->buildCudaEngine(*n);
    return create_engine(engine);
}