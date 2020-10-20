//
// Created by mason on 10/10/20.
//

#include "TRTTensorInternal.hpp"
#include "../TRTDims/TRTDimsInternal.hpp"

const char* tensor_get_name(Tensor_t *tensor) {
    return tensor->internal_tensor->getName();
}

void tensor_set_dimensions(Tensor_t *tensor, Dims_t *dimensions) {
    if (tensor == nullptr) {
        return;
    }

    tensor->internal_tensor->setDimensions(dims_get(dimensions));
}

void tensor_destroy(Tensor_t *tensor) {
    delete tensor;
}

