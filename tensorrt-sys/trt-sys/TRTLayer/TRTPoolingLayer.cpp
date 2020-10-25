//
// Created by mason on 10/13/20.
//

#include "TRTPoolingLayer.h"
#include "../TRTDims/TRTDimsInternal.hpp"


void pooling_set_pooling_type(nvinfer1::IPoolingLayer *layer, PoolingType poolingType) {
    layer->setPoolingType(static_cast<nvinfer1::PoolingType>(poolingType));
}

PoolingType pooling_get_pooling_type(nvinfer1::IPoolingLayer *layer) {
    return static_cast<PoolingType>(layer->getPoolingType());
}

void pooling_set_window_size(nvinfer1::IPoolingLayer *layer, Dims_t *dims) {
    layer->setWindowSize(dimsHW_get(dims));
}

Dims_t *pooling_get_window_size(nvinfer1::IPoolingLayer *layer) {
    auto dim = layer->getWindowSize();
    return create_dimsHW(dim.h(), dim.w());
}

void pooling_set_stride(nvinfer1::IPoolingLayer *layer, Dims_t *stride) {
    layer->setStride(dimsHW_get(stride));
}

Dims_t *pooling_get_stride(nvinfer1::IPoolingLayer *layer) {
    auto stride = layer->getStride();
    return create_dimsHW(stride.h(), stride.w());
}

void pooling_set_padding(nvinfer1::IPoolingLayer *layer, Dims_t *padding) {
    layer->setPadding(dimsHW_get(padding));
}

Dims_t *pooling_get_padding(nvinfer1::IPoolingLayer *layer) {
    auto padding = layer->getPadding();
    return create_dimsHW(padding.h(), padding.w());
}

void pooling_set_blend_factor(nvinfer1::IPoolingLayer *layer, float factor) {
   layer->setBlendFactor(factor);
}

float pooling_get_blend_factor(nvinfer1::IPoolingLayer *layer) {
    return layer->getBlendFactor();
}

void pooling_set_average_count_excludes_padding(nvinfer1::IPoolingLayer *layer, bool exclusive) {
    layer->setAverageCountExcludesPadding(exclusive);
}

bool pooling_get_average_count_excludes_padding(nvinfer1::IPoolingLayer *layer) {
    return layer->getAverageCountExcludesPadding();
}

void pooling_set_pre_padding(nvinfer1::IPoolingLayer *layer, Dims_t *pre_padding) {
    layer->setPrePadding(dims_get(pre_padding));
}

Dims_t *pooling_get_pre_padding(nvinfer1::IPoolingLayer *layer) {
    auto prePadding = layer->getPrePadding();
    return create_dims(prePadding.nbDims, prePadding.d, reinterpret_cast<DimensionType_t*>(prePadding.type));
}

void pooling_set_post_padding(nvinfer1::IPoolingLayer *layer, Dims_t *post_padding) {
    layer->setPrePadding(dims_get(post_padding));
}

Dims_t *pooling_get_post_padding(nvinfer1::IPoolingLayer *layer) {
    auto postPadding = layer->getPrePadding();
    return create_dims(postPadding.nbDims, postPadding.d, reinterpret_cast<DimensionType_t*>(postPadding.type));
}

void pooling_set_padding_mode(nvinfer1::IPoolingLayer *layer, PaddingMode mode) {
    layer->setPaddingMode(static_cast<nvinfer1::PaddingMode>(mode));
}

PaddingMode pooling_get_padding_mode(nvinfer1::IPoolingLayer *layer) {
    return static_cast<PaddingMode>(layer->getPaddingMode());
}
