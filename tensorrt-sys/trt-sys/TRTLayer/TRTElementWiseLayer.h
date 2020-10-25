//
// Created by mason on 10/12/20.
//

#ifndef LIBTRT_TRTELEMENTWISELAYER_H
#define LIBTRT_TRTELEMENTWISELAYER_H

#include "TRTLayer.h"
#include "../TRTEnums.h"

void elementwise_set_operation(nvinfer1::IElementWiseLayer *element_wise_layer, ElementWiseOperation_t type);
ElementWiseOperation_t elementwise_get_operation(nvinfer1::IElementWiseLayer *element_wise_layer);

#endif //LIBTRT_TRTELEMENTWISELAYER_H
