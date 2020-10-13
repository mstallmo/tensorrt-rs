//
// Created by mason on 10/12/20.
//

#include "TRTGatherLayer.h"
#include "TRTLayerInternal.hpp"

int32_t gather_layer_get_gather_axis(Layer_t *layer) {
    auto concrete = dynamic_cast<nvinfer1::IGatherLayer*>(layer->internal_layer);
    return concrete->getGatherAxis();
}

void gather_layer_set_gather_axis(Layer_t *layer, int32_t axis) {
    auto concrete = dynamic_cast<nvinfer1::IGatherLayer*>(layer->internal_layer);
    concrete->setGatherAxis(axis);
}

void gather_layer_destroy(Layer_t *layer) {
    delete layer;
}
