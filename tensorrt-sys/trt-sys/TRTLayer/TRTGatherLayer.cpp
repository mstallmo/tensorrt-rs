//
// Created by mason on 10/12/20.
//

#include "TRTGatherLayer.h"

int32_t gather_layer_get_gather_axis(nvinfer1::IGatherLayer *layer) {
    return layer->getGatherAxis();
}

void gather_layer_set_gather_axis(nvinfer1::IGatherLayer *layer, int32_t axis) {
    layer->setGatherAxis(axis);
}
