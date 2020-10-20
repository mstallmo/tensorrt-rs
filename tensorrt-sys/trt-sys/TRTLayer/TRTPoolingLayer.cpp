//
// Created by mason on 10/13/20.
//

#include "TRTPoolingLayer.h"
#include "TRTLayerInternal.hpp"
#include "../TRTDims/TRTDimsInternal.hpp"


void pooling_set_pooling_type(Layer_t *layer, PoolingType poolingType) {
    auto concrete = dynamic_cast<nvinfer1::IPoolingLayer*>(layer->internal_layer);
    concrete->setPoolingType(static_cast<nvinfer1::PoolingType>(poolingType));
}

PoolingType pooling_get_pooling_type(Layer_t *layer) {
    auto concrete = dynamic_cast<nvinfer1::IPoolingLayer*>(layer->internal_layer);
    return static_cast<PoolingType>(concrete->getPoolingType());
}

void pooling_set_window_size(Layer_t *layer, Dims_t *dims) {
    auto concrete = dynamic_cast<nvinfer1::IPoolingLayer*>(layer->internal_layer);
    concrete->setWindowSize(dimsHW_get(dims));
}

Dims_t *pooling_get_window_size(Layer_t *layer) {
    auto concrete = dynamic_cast<nvinfer1::IPoolingLayer*>(layer->internal_layer);
    auto dim = concrete->getWindowSize();
    return create_dimsHW(dim.h(), dim.w());
}

void pooling_set_stride(Layer_t *layer, Dims_t *stride) {
    auto concrete = dynamic_cast<nvinfer1::IPoolingLayer*>(layer->internal_layer);
    concrete->setStride(dimsHW_get(stride));
}

Dims_t *pooling_get_stride(Layer_t *layer) {
    auto concrete = dynamic_cast<nvinfer1::IPoolingLayer*>(layer->internal_layer);
    auto stride = concrete->getStride();
    return create_dimsHW(stride.h(), stride.w());
}

void pooling_set_padding(Layer_t *layer, Dims_t *padding) {
    auto concrete = dynamic_cast<nvinfer1::IPoolingLayer*>(layer->internal_layer);
    concrete->setPadding(dimsHW_get(padding));
}

Dims_t *pooling_get_padding(Layer_t *layer) {
    auto concrete = dynamic_cast<nvinfer1::IPoolingLayer*>(layer->internal_layer);
    auto padding = concrete->getPadding();
    return create_dimsHW(padding.h(), padding.w());
}

void pooling_set_blend_factor(Layer_t *layer, float factor) {
   auto concrete = dynamic_cast<nvinfer1::IPoolingLayer*>(layer->internal_layer);
   concrete->setBlendFactor(factor);
}

float pooling_get_blend_factor(Layer_t *layer) {
    auto concrete = dynamic_cast<nvinfer1::IPoolingLayer*>(layer->internal_layer);
    return concrete->getBlendFactor();
}

void pooling_set_average_count_excludes_padding(Layer_t *layer, bool exclusive) {
    auto concrete = dynamic_cast<nvinfer1::IPoolingLayer*>(layer->internal_layer);
    concrete->setAverageCountExcludesPadding(exclusive);
}

bool pooling_get_average_count_excludes_padding(Layer_t *layer) {
    auto concrete = dynamic_cast<nvinfer1::IPoolingLayer*>(layer->internal_layer);
    return concrete->getAverageCountExcludesPadding();
}

void pooling_set_pre_padding(Layer_t *layer, Dims_t *pre_padding) {
    auto concrete = dynamic_cast<nvinfer1::IPoolingLayer*>(layer->internal_layer);
    concrete->setPrePadding(dims_get(pre_padding));
}

Dims_t *pooling_get_pre_padding(Layer_t *layer) {
    auto concrete = dynamic_cast<nvinfer1::IPoolingLayer*>(layer->internal_layer);
    auto prePadding = concrete->getPrePadding();
    return create_dims(prePadding.nbDims, prePadding.d, reinterpret_cast<DimensionType_t*>(prePadding.type));
}

void pooling_set_post_padding(Layer_t *layer, Dims_t *post_padding) {
    auto concrete = dynamic_cast<nvinfer1::IPoolingLayer*>(layer->internal_layer);
    concrete->setPrePadding(dims_get(post_padding));
}

Dims_t *pooling_get_post_padding(Layer_t *layer) {
    auto concrete = dynamic_cast<nvinfer1::IPoolingLayer*>(layer->internal_layer);
    auto postPadding = concrete->getPrePadding();
    return create_dims(postPadding.nbDims, postPadding.d, reinterpret_cast<DimensionType_t*>(postPadding.type));
}

void pooling_set_padding_mode(Layer_t *layer, PaddingMode mode) {
    auto concrete = dynamic_cast<nvinfer1::IPoolingLayer*>(layer->internal_layer);
    concrete->setPaddingMode(static_cast<nvinfer1::PaddingMode>(mode));
}

PaddingMode pooling_get_padding_mode(Layer_t *layer) {
    auto concrete = dynamic_cast<nvinfer1::IPoolingLayer*>(layer->internal_layer);
    return static_cast<PaddingMode>(concrete->getPaddingMode());
}

void pooling_destroy(Layer_t *layer) {
    delete layer;
}