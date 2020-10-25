//
// Created by mason on 10/10/20.
//

#ifndef LIBTRT_TRTTENSOR_H
#define LIBTRT_TRTTENSOR_H

#include <NvInfer.h>
#include "../TRTDims/TRTDims.h"

const char* tensor_get_name(nvinfer1::ITensor *tensor);
void tensor_set_dimensions(nvinfer1::ITensor *tensor, Dims_t *dimensions); // only valid for input tensors

#endif //LIBTRT_TRTTENSOR_H
