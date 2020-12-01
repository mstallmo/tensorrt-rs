//
// Created by mason on 10/10/20.
//

#include "TRTTensor.h"

const char* tensor_get_name(nvinfer1::ITensor *tensor) {
    return tensor->getName();
}

void tensor_set_dimensions(nvinfer1::ITensor *tensor, nvinfer1::Dims dimensions) {
    tensor->setDimensions(dimensions);
}

