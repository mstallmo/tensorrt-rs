//
// Created by mason on 10/10/20.
//

#ifndef LIBTRT_TRTTENSOR_H
#define LIBTRT_TRTTENSOR_H


#include "../TRTDims/TRTDims.h"

#ifdef __cplusplus
extern "C" {
#endif
struct Tensor;
typedef struct Tensor Tensor_t;

const char* tensor_get_name(Tensor_t*);
void tensor_set_dimensions(Tensor_t *tensor, Dims_t *dimensions); // only valid for input tensors

void tensor_destroy(Tensor_t *tensor);

#ifdef __cplusplus
};
#endif

#endif //LIBTRT_TRTTENSOR_H
