//
// Created by mason on 10/7/20.
//
#include <NvInfer.h>

#include "TRTLayerInternal.hpp"

const char* layer_get_name(Layer_t *layer) {
    if(layer == nullptr) {
        return nullptr;
    }

    return layer->internal_layer->getName();
}


