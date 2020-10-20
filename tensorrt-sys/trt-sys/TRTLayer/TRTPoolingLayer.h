//
// Created by mason on 10/13/20.
//

#ifndef LIBTRT_TRTPOOLINGLAYER_H
#define LIBTRT_TRTPOOLINGLAYER_H

#include "../TRTEnums.h"
#include "TRTLayer.h"

#ifdef __cplusplus
extern "C" {
#endif

void pooling_set_pooling_type(Layer_t *layer, PoolingType poolingType);
PoolingType pooling_get_pooling_type(Layer_t *layer);

void pooling_set_window_size(Layer_t *layer, Dims_t *dims);
Dims_t *pooling_get_window_size(Layer_t *layer);

void pooling_set_stride(Layer_t *layer, Dims_t *stride);
Dims_t *pooling_get_stride(Layer_t *layer);

void pooling_set_padding(Layer_t *layer, Dims_t *padding);
Dims_t *pooling_get_padding(Layer_t *layer);

void pooling_set_blend_factor(Layer_t *layer, float factor);
float pooling_get_blend_factor(Layer_t *layer);

void pooling_set_average_count_excludes_padding(Layer_t *layer, bool exclusive);
bool pooling_get_average_count_excludes_padding(Layer_t *layer);

void pooling_set_pre_padding(Layer_t *layer, Dims_t *pre_padding);
Dims_t *pooling_get_pre_padding(Layer_t *layer);

void pooling_set_post_padding(Layer_t *layer, Dims_t *post_padding);
Dims_t *pooling_get_post_padding(Layer_t *layer);

void pooling_set_padding_mode(Layer_t *layer, PaddingMode mode);
PaddingMode pooling_get_padding_mode(Layer_t *layer);

void pooling_destroy(Layer_t *layer);


#ifdef __cplusplus
};
#endif

#endif //LIBTRT_TRTPOOLINGLAYER_H
