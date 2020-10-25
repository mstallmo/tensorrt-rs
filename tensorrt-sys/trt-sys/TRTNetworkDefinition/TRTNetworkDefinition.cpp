//
// Created by mason on 11/27/19.
//
#include <NvInfer.h>

#include "../TRTLayer/TRTLayerInternal.hpp"
#include "../TRTDims/TRTDimsInternal.hpp"

void destroy_network(nvinfer1::INetworkDefinition *network) {
    network->destroy();
}

nvinfer1::ITensor *network_add_input(nvinfer1::INetworkDefinition *network, const char *name, DataType_t type, Dims_t *dims) {
    return network->addInput(name, static_cast<nvinfer1::DataType>(type), dims_get(dims));
}

nvinfer1::ITensor *network_get_input(nvinfer1::INetworkDefinition *network, int32_t idx) {
    return network->getInput(idx);
}

int network_get_nb_layers(nvinfer1::INetworkDefinition *network) {
    return network->getNbLayers();
}

Layer_t *network_get_layer(nvinfer1::INetworkDefinition *network, int index) {
    return new Layer(network->getLayer(index));
}

Layer_t *network_add_identity_layer(nvinfer1::INetworkDefinition *network, nvinfer1::ITensor *inputTensor) {
    return new Layer(network->addIdentity(*inputTensor));
}

int network_get_nb_inputs(nvinfer1::INetworkDefinition *network) {
    return network->getNbInputs();
}

int network_get_nb_outputs(nvinfer1::INetworkDefinition *network) {
    return network->getNbOutputs();
}

nvinfer1::ITensor *network_get_output(nvinfer1::INetworkDefinition *network, int32_t index) {
    return network->getOutput(index);
}

void network_remove_tensor(nvinfer1::INetworkDefinition *network, nvinfer1::ITensor *tensor) {
    network->removeTensor(*tensor);
}

void network_mark_output(nvinfer1::INetworkDefinition *network, nvinfer1::ITensor *tensor) {
    network->markOutput(*tensor);
}

void network_unmark_output(nvinfer1::INetworkDefinition *network, nvinfer1::ITensor *tensor) {
    network->unmarkOutput(*tensor);
}

Layer_t *network_add_element_wise(nvinfer1::INetworkDefinition *network, nvinfer1::ITensor *input1, nvinfer1::ITensor *input2, ElementWiseOperation_t op) {
    return new Layer(network->addElementWise(*input1, *input2,
                                                               static_cast<nvinfer1::ElementWiseOperation>(op)));
}

Layer_t *network_add_gather(nvinfer1::INetworkDefinition *network, nvinfer1::ITensor *data, nvinfer1::ITensor *indices, int32_t axis) {
    return new Layer(network->addGather(*data, *indices, axis));
}

Layer_t *network_add_activation(nvinfer1::INetworkDefinition *network, nvinfer1::ITensor *input, ActivationType_t type) {
    return new Layer(network->addActivation(*input,
                                                              static_cast<nvinfer1::ActivationType>(type)));
}

Layer_t *network_add_pooling(nvinfer1::INetworkDefinition *network, nvinfer1::ITensor *input, PoolingType poolingType, Dims_t *dims) {
    return new Layer(network->addPooling(*input,
                                                           static_cast<nvinfer1::PoolingType>(poolingType),
                                                           dimsHW_get(dims)));
}