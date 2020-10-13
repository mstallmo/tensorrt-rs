//
// Created by mason on 10/12/20.
//

#include <NvInfer.h>

#include "TRTElementWiseLayer.h"
#include "TRTLayerInternal.hpp"

void elementwise_set_operation(Layer_t *element_wise_layer, ElementWiseOperation_t operation) {
    auto concrete_layer = dynamic_cast<nvinfer1::IElementWiseLayer*>(element_wise_layer->internal_layer);
    concrete_layer->setOperation(static_cast<nvinfer1::ElementWiseOperation>(operation));
}

ElementWiseOperation_t elementwise_get_operation(Layer_t *element_wise_layer) {
    auto concrete_layer = dynamic_cast<nvinfer1::IElementWiseLayer*>(element_wise_layer->internal_layer);
    return static_cast<ElementWiseOperation_t>(concrete_layer->getOperation());
}

void elementwise_destroy(Layer_t *element_wise_layer) {
    delete element_wise_layer;
}