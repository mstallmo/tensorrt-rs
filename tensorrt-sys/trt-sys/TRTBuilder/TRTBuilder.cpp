//
// Created by mason on 11/27/19.
//
#include <memory>
#include <NvInfer.h>
#include <NvInferPlugin.h>

#include "TRTBuilder.h"
#include "../TRTNetworkDefinition/TRTNetworkDefinitionInternal.hpp"
#include "../TRTLogger/TRTLoggerInternal.hpp"
#include "../TRTCudaEngine/TRTCudaEngineInternal.hpp"
#include "../TRTUtils.hpp"

#define MAX_WORKSPACE (1 << 30)

struct Builder {
    using IBuilderPtr = std::unique_ptr<nvinfer1::IBuilder, TRTDeleter<nvinfer1::IBuilder>>;
    IBuilderPtr internal_builder;

    explicit Builder(nvinfer1::ILogger& logger) {
        internal_builder = IBuilderPtr(nvinfer1::createInferBuilder(logger));
        internal_builder->setMaxBatchSize(1);
        internal_builder->setMaxWorkspaceSize(MAX_WORKSPACE);
    };
};


void builder_set_max_batch_size(Builder_t* builder, int32_t batch_size) {
    if (builder == nullptr)
        return;

	builder->internal_builder->setMaxBatchSize(batch_size);
}

int32_t builder_get_max_batch_size(Builder_t* builder) {
    if (builder == nullptr)
        return -1;

	return builder->internal_builder->getMaxBatchSize();
}

void builder_set_max_workspace_size(Builder_t* builder, size_t workspace_size) {
    if (builder == nullptr)
        return;

	builder->internal_builder->setMaxWorkspaceSize(workspace_size);
}

size_t builder_get_max_workspace_size(Builder_t* builder) {
    if (builder == nullptr)
        return -1;

	return builder->internal_builder->getMaxWorkspaceSize();
}

Builder_t *create_infer_builder(Logger_t *logger) {
    initLibNvInferPlugins(&logger->getLogger(), "");

    return new Builder(logger->getLogger());
}

void destroy_builder(Builder_t *builder) {
    if (builder == nullptr)
        return;

    delete builder;
}

Network_t *create_network(Builder_t *builder) {
    if (builder == nullptr)
        return nullptr;

    return new Network(builder->internal_builder->createNetwork());
}

Network_t *create_network_v2(Builder_t *builder, uint32_t flags) {
    if (builder == nullptr)
        return nullptr;

    return new Network(builder->internal_builder->createNetworkV2(flags));
}

Engine_t *build_cuda_engine(Builder_t *builder, Network_t *network) {
    if (builder == nullptr || network == nullptr)
        return nullptr;

    auto& b = builder->internal_builder;

    auto engine = b->buildCudaEngine(network->getNetworkDefinition());
    return create_engine(engine);
}
