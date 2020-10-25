//
// Created by mason on 10/7/20.
//
#include <NvInfer.h>

#include "TRTLayerInternal.hpp"

LayerType_t layer_get_type(Layer_t *layer) {
    return static_cast<LayerType_t>(layer->internal_layer->getType());
}

void layer_set_name(Layer_t* layer, const char* name) {
    layer->internal_layer->setName(name);
}

const char* layer_get_name(Layer_t *layer) {
    if(layer == nullptr) {
        return nullptr;
    }

    return layer->internal_layer->getName();
}

int32_t layer_get_nb_inputs(Layer_t *layer) {
    return layer->internal_layer->getNbInputs();
}

nvinfer1::ITensor* layer_get_input(Layer_t *layer, int32_t index) {
    return layer->internal_layer->getInput(index);
}

int32_t layer_get_nb_outputs(Layer_t *layer) {
    return layer->internal_layer->getNbOutputs();
}

nvinfer1::ITensor* layer_get_output(Layer_t *layer, int32_t index) {
    return layer->internal_layer->getOutput(index);
}

void layer_set_input(Layer_t *layer, int32_t index, nvinfer1::ITensor *tensor) {
    layer->internal_layer->setInput(index, *tensor);
}

void layer_set_precision(Layer_t *layer, DataType_t precision) {
    layer->internal_layer->setPrecision(static_cast<nvinfer1::DataType>(precision));
}

DataType_t layer_get_precision(Layer_t *layer) {
    return static_cast<DataType_t>(layer->internal_layer->getPrecision());
}

bool layer_precision_is_set(Layer_t *layer) {
    return layer->internal_layer->precisionIsSet();
}

void layer_reset_precision(Layer_t *layer) {
    layer->internal_layer->resetPrecision();
}

void layer_set_output_type(Layer_t *layer, int32_t index, DataType_t dataType) {
    layer->internal_layer->setOutputType(index, static_cast<nvinfer1::DataType>(dataType));
}

DataType_t layer_get_output_type(Layer_t *layer, int32_t index) {
    return static_cast<DataType_t>(layer->internal_layer->getOutputType(index));
}

bool layer_output_type_is_set(Layer_t *layer, int32_t index) {
    return layer->internal_layer->outputTypeIsSet(index);
}

void layer_reset_output_type(Layer_t *layer, int32_t index) {
    layer->internal_layer->resetOutputType(index);
}
