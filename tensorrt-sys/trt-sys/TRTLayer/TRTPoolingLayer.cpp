//
// Created by mason on 10/13/20.
//

#include "TRTPoolingLayer.h"


void pooling_set_pooling_type(nvinfer1::IPoolingLayer *layer, PoolingType poolingType) {
    layer->setPoolingType(static_cast<nvinfer1::PoolingType>(poolingType));
}

PoolingType pooling_get_pooling_type(nvinfer1::IPoolingLayer *layer) {
    return static_cast<PoolingType>(layer->getPoolingType());
}

void pooling_set_window_size(nvinfer1::IPoolingLayer *layer, nvinfer1::DimsHW dims) {
    layer->setWindowSize(dims);
}

nvinfer1::DimsHW pooling_get_window_size(nvinfer1::IPoolingLayer *layer) {
    return layer->getWindowSize();
}

void pooling_set_stride(nvinfer1::IPoolingLayer *layer, nvinfer1::DimsHW stride) {
    layer->setStride(stride);
}

nvinfer1::DimsHW pooling_get_stride(nvinfer1::IPoolingLayer *layer) {
    return layer->getStride();
}

void pooling_set_padding(nvinfer1::IPoolingLayer *layer, nvinfer1::DimsHW padding) {
    layer->setPadding(padding);
}

nvinfer1::DimsHW pooling_get_padding(nvinfer1::IPoolingLayer *layer) {
    return layer->getPadding();
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

void pooling_set_pre_padding(nvinfer1::IPoolingLayer *layer, nvinfer1::Dims pre_padding) {
    layer->setPrePadding(pre_padding);
}

nvinfer1::Dims pooling_get_pre_padding(nvinfer1::IPoolingLayer *layer) {
    return layer->getPrePadding();
}

void pooling_set_post_padding(nvinfer1::IPoolingLayer *layer, nvinfer1::Dims post_padding) {
    layer->setPrePadding(post_padding);
}

nvinfer1::Dims pooling_get_post_padding(nvinfer1::IPoolingLayer *layer) {
    return layer->getPrePadding();
}

void pooling_set_padding_mode(nvinfer1::IPoolingLayer *layer, PaddingMode mode) {
    layer->setPaddingMode(static_cast<nvinfer1::PaddingMode>(mode));
}

PaddingMode pooling_get_padding_mode(nvinfer1::IPoolingLayer *layer) {
    return static_cast<PaddingMode>(layer->getPaddingMode());
}
