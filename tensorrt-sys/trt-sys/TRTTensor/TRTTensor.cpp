//
// Created by mason on 10/10/20.
//

#include "TRTTensorInternal.hpp"
#include "../TRTDims/TRTDimsInternal.hpp"


void tensor_set_dimensions(Tensor_t *tensor, Dims_t *dimensions) {
    if (tensor == nullptr) {
        return;
    }

    tensor->internal_tensor->setDimensions(dims_get(dimensions));
}

