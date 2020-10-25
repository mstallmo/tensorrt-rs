//
// Created by mason on 10/10/20.
//

#include "TRTTensor.h"
#include "../TRTDims/TRTDimsInternal.hpp"

const char* tensor_get_name(nvinfer1::ITensor *tensor) {
    return tensor->getName();
}

void tensor_set_dimensions(nvinfer1::ITensor *tensor, Dims_t *dimensions) {
    tensor->setDimensions(dims_get(dimensions));
}

