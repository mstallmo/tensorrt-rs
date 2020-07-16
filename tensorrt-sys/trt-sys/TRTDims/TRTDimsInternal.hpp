//
// Created by mason on 4/30/20.
//

#ifndef LIBTRT_TRTDIMSINTERNAL_HPP
#define LIBTRT_TRTDIMSINTERNAL_HPP

#include <NvInfer.h>

#include "TRTDims.h"

nvinfer1::Dims dims_get(const Dims_t* dims);
Dims_t* dims_create(nvinfer1::Dims dim);

#endif //LIBTRT_TRTDIMSINTERNAL_HPP
