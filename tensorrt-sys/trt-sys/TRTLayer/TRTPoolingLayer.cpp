//
// Created by mason on 10/13/20.
//

#include "TRTPoolingLayer.h"
#include "TRTLayerInternal.hpp"


void pooling_destroy(Layer_t *layer) {
    delete layer;
}