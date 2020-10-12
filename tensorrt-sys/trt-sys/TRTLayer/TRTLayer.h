//
// Created by mason on 10/7/20.
//

#ifndef LIBTRT_TRTLAYER_H
#define LIBTRT_TRTLAYER_H

#include "../TRTTensor/TRTTensor.h"

#ifdef __cplusplus
extern "C" {
#endif

struct Layer;
typedef struct Layer Layer_t;

const char* layer_get_name(Layer_t *layer);

int layer_get_nb_inputs(Layer_t *layer);

Tensor_t* layer_get_input(Layer_t *layer, int32_t index);

Tensor_t * layer_get_output(Layer_t *layer, int32_t index);

void layer_set_input(Layer_t *layer, int32_t index, Tensor_t *tensor);

#ifdef __cplusplus
};
#endif

#endif //LIBTRT_TRTLAYER_H
