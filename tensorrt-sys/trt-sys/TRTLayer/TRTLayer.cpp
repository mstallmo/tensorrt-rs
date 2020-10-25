//
// Created by mason on 10/7/20.
//
#include <NvInfer.h>
#include "TRTLayer.h"

LayerType_t layer_get_type(nvinfer1::ILayer *layer) {
    return static_cast<LayerType_t>(layer->getType());
}

void layer_set_name(nvinfer1::ILayer* layer, const char* name) {
    layer->setName(name);
}

const char* layer_get_name(nvinfer1::ILayer *layer) {
    if(layer == nullptr) {
        return nullptr;
    }

    return layer->getName();
}

int32_t layer_get_nb_inputs(nvinfer1::ILayer *layer) {
    return layer->getNbInputs();
}

nvinfer1::ITensor* layer_get_input(nvinfer1::ILayer *layer, int32_t index) {
    return layer->getInput(index);
}

int32_t layer_get_nb_outputs(nvinfer1::ILayer *layer) {
    return layer->getNbOutputs();
}

nvinfer1::ITensor* layer_get_output(nvinfer1::ILayer *layer, int32_t index) {
    return layer->getOutput(index);
}

void layer_set_input(nvinfer1::ILayer *layer, int32_t index, nvinfer1::ITensor *tensor) {
    layer->setInput(index, *tensor);
}

void layer_set_precision(nvinfer1::ILayer *layer, DataType_t precision) {
    layer->setPrecision(static_cast<nvinfer1::DataType>(precision));
}

DataType_t layer_get_precision(nvinfer1::ILayer *layer) {
    return static_cast<DataType_t>(layer->getPrecision());
}

bool layer_precision_is_set(nvinfer1::ILayer *layer) {
    return layer->precisionIsSet();
}

void layer_reset_precision(nvinfer1::ILayer *layer) {
    layer->resetPrecision();
}

void layer_set_output_type(nvinfer1::ILayer *layer, int32_t index, DataType_t dataType) {
    layer->setOutputType(index, static_cast<nvinfer1::DataType>(dataType));
}

DataType_t layer_get_output_type(nvinfer1::ILayer *layer, int32_t index) {
    return static_cast<DataType_t>(layer->getOutputType(index));
}

bool layer_output_type_is_set(nvinfer1::ILayer *layer, int32_t index) {
    return layer->outputTypeIsSet(index);
}

void layer_reset_output_type(nvinfer1::ILayer *layer, int32_t index) {
    layer->resetOutputType(index);
}
