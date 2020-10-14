//
// Created by mason on 10/13/20.
//

#ifndef LIBTRT_TRTACTIVATIONLAYER_H
#define LIBTRT_TRTACTIVATIONLAYER_H

#include "../TRTEnums.h"
#include "TRTLayer.h"

#ifdef __cplusplus
extern "C" {
#endif

void activation_set_activation_type(Layer_t *layer, ActivationType_t activationType);

ActivationType_t activation_get_activation_type(Layer_t *layer);

void activation_set_alpha(Layer_t *layer, float alpha);

float activation_get_alpha(Layer_t *layer);

void activation_set_beta(Layer_t *layer, float beta);

float activation_get_beta(Layer_t *layer);

void activation_destroy(Layer_t *layer);

#ifdef __cplusplus
};
#endif

#endif //LIBTRT_TRTACTIVATIONLAYER_H
