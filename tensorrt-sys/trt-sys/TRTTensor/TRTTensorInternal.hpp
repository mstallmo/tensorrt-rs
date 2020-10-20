//
// Created by mason on 10/10/20.
//

#ifndef LIBTRT_TRTTENSORINTERNAL_HPP
#define LIBTRT_TRTTENSORINTERNAL_HPP

#include <NvInfer.h>
#include "TRTTensor.h"

struct Tensor {
    nvinfer1::ITensor *internal_tensor;

    explicit Tensor(nvinfer1::ITensor *tensor) : internal_tensor(tensor) {};
};

#endif //LIBTRT_TRTTENSORINTERNAL_HPP
