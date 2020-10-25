//
// Created by mason on 10/7/20.
//

#ifndef LIBTRT_TRTLAYER_H
#define LIBTRT_TRTLAYER_H

#include <stdint.h>
#include "../TRTTensor/TRTTensor.h"
#include "../TRTEnums.h"

#ifdef __cplusplus
extern "C" {
#endif

struct Layer;
typedef struct Layer Layer_t;

LayerType_t layer_get_type(Layer_t *layer);

void layer_set_name(Layer_t *layer, const char* name);

const char* layer_get_name(Layer_t *layer);

int32_t layer_get_nb_inputs(Layer_t *layer);

nvinfer1::ITensor* layer_get_input(Layer_t *layer, int32_t index);

int32_t layer_get_nb_outputs(Layer_t *layer);

nvinfer1::ITensor * layer_get_output(Layer_t *layer, int32_t index);

void layer_set_input(Layer_t *layer, int32_t index, nvinfer1::ITensor *tensor);

void layer_set_precision(Layer_t *layer, DataType_t precision);

DataType_t layer_get_precision(Layer_t *layer);

bool layer_precision_is_set(Layer_t *layer);

void layer_reset_precision(Layer_t *layer);

void layer_set_output_type(Layer_t *layer, int32_t index, DataType_t dataType);

DataType_t layer_get_output_type(Layer_t *layer, int32_t index);

bool layer_output_type_is_set(Layer_t *layer, int32_t index);

void layer_reset_output_type(Layer_t *layer, int32_t index);

#ifdef __cplusplus
};
#endif

#endif //LIBTRT_TRTLAYER_H
