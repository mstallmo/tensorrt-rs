//
// Created by mason on 10/7/20.
//
#include <NvInfer.h>

#include "TRTLayerInternal.hpp"
#include "../TRTTensor/TRTTensorInternal.hpp"

const char* layer_get_name(Layer_t *layer) {
    if(layer == nullptr) {
        return nullptr;
    }

    return layer->internal_layer->getName();
}

int layer_get_nb_inputs(Layer_t *layer) {
    return layer->internal_layer->getNbInputs();
}

Tensor_t* layer_get_input(Layer_t *layer, int32_t index) {
    return new Tensor(layer->internal_layer->getInput(index));
}

Tensor_t* layer_get_output(Layer_t *layer, int32_t index) {
    return new Tensor(layer->internal_layer->getOutput(index));
}

void layer_set_input(Layer_t *layer, int32_t index, Tensor_t *tensor) {
    layer->internal_layer->setInput(index, *tensor->internal_tensor);
}