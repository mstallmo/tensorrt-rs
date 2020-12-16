//
// Created by mason on 10/12/20.
//

#include <NvInfer.h>

#include "TRTElementWiseLayer.h"

void elementwise_set_operation(nvinfer1::IElementWiseLayer *element_wise_layer, ElementWiseOperation_t operation) {
    element_wise_layer->setOperation(static_cast<nvinfer1::ElementWiseOperation>(operation));
}

ElementWiseOperation_t elementwise_get_operation(nvinfer1::IElementWiseLayer *element_wise_layer) {
    return static_cast<ElementWiseOperation_t>(element_wise_layer->getOperation());
}
