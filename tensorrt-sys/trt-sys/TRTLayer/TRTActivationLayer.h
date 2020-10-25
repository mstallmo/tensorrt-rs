//
// Created by mason on 10/13/20.
//

#ifndef LIBTRT_TRTACTIVATIONLAYER_H
#define LIBTRT_TRTACTIVATIONLAYER_H

#include "../TRTEnums.h"
#include "TRTLayer.h"

void activation_set_activation_type(nvinfer1::IActivationLayer *layer, ActivationType_t activationType);
ActivationType_t activation_get_activation_type(nvinfer1::IActivationLayer *layer);
void activation_set_alpha(nvinfer1::IActivationLayer *layer, float alpha);
float activation_get_alpha(nvinfer1::IActivationLayer *layer);
void activation_set_beta(nvinfer1::IActivationLayer *layer, float beta);
float activation_get_beta(nvinfer1::IActivationLayer *layer);

#endif //LIBTRT_TRTACTIVATIONLAYER_H
