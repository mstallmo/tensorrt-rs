//
// Created by mason on 10/7/20.
//

#ifndef LIBTRT_TRTLAYER_H
#define LIBTRT_TRTLAYER_H

#include <NvInfer.h>
#include <stdint.h>
#include "../TRTTensor/TRTTensor.h"
#include "../TRTEnums.h"

LayerType_t layer_get_type(nvinfer1::ILayer *layer);
void layer_set_name(nvinfer1::ILayer *layer, const char* name);
const char* layer_get_name(nvinfer1::ILayer *layer);
int32_t layer_get_nb_inputs(nvinfer1::ILayer *layer);
nvinfer1::ITensor* layer_get_input(nvinfer1::ILayer *layer, int32_t index);
int32_t layer_get_nb_outputs(nvinfer1::ILayer *layer);
nvinfer1::ITensor * layer_get_output(nvinfer1::ILayer *layer, int32_t index);
void layer_set_input(nvinfer1::ILayer *layer, int32_t index, nvinfer1::ITensor *tensor);
void layer_set_precision(nvinfer1::ILayer *layer, DataType_t precision);
DataType_t layer_get_precision(nvinfer1::ILayer *layer);
bool layer_precision_is_set(nvinfer1::ILayer *layer);
void layer_reset_precision(nvinfer1::ILayer *layer);
void layer_set_output_type(nvinfer1::ILayer *layer, int32_t index, DataType_t dataType);
DataType_t layer_get_output_type(nvinfer1::ILayer *layer, int32_t index);
bool layer_output_type_is_set(nvinfer1::ILayer *layer, int32_t index);
void layer_reset_output_type(nvinfer1::ILayer *layer, int32_t index);

#endif //LIBTRT_TRTLAYER_H
