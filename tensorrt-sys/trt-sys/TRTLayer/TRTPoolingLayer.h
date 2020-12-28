//
// Created by mason on 10/13/20.
//

#ifndef LIBTRT_TRTPOOLINGLAYER_H
#define LIBTRT_TRTPOOLINGLAYER_H

#include "../TRTEnums.h"
#include "../TRTDims/TRTDims.h"
#include "TRTLayer.h"

void pooling_set_pooling_type(nvinfer1::IPoolingLayer *layer, PoolingType poolingType);
PoolingType pooling_get_pooling_type(nvinfer1::IPoolingLayer *layer);

void pooling_set_window_size(nvinfer1::IPoolingLayer *layer, nvinfer1::DimsHW dims);
nvinfer1::DimsHW pooling_get_window_size(nvinfer1::IPoolingLayer *layer);

void pooling_set_stride(nvinfer1::IPoolingLayer *layer, nvinfer1::DimsHW stride);
nvinfer1::DimsHW pooling_get_stride(nvinfer1::IPoolingLayer *layer);

void pooling_set_padding(nvinfer1::IPoolingLayer *layer, nvinfer1::DimsHW padding);
nvinfer1::DimsHW pooling_get_padding(nvinfer1::IPoolingLayer *layer);

void pooling_set_blend_factor(nvinfer1::IPoolingLayer *layer, float factor);
float pooling_get_blend_factor(nvinfer1::IPoolingLayer *layer);

void pooling_set_average_count_excludes_padding(nvinfer1::IPoolingLayer *layer, bool exclusive);
bool pooling_get_average_count_excludes_padding(nvinfer1::IPoolingLayer *layer);

void pooling_set_pre_padding(nvinfer1::IPoolingLayer *layer, nvinfer1::Dims pre_padding);
nvinfer1::Dims pooling_get_pre_padding(nvinfer1::IPoolingLayer *layer);

void pooling_set_post_padding(nvinfer1::IPoolingLayer *layer, nvinfer1::Dims post_padding);
nvinfer1::Dims pooling_get_post_padding(nvinfer1::IPoolingLayer *layer);

void pooling_set_padding_mode(nvinfer1::IPoolingLayer *layer, PaddingMode mode);
PaddingMode pooling_get_padding_mode(nvinfer1::IPoolingLayer *layer);

#endif //LIBTRT_TRTPOOLINGLAYER_H
