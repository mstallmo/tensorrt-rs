//
// Created by mason on 10/13/20.
//

#include "TRTActivationLayer.h"

void activation_set_activation_type(nvinfer1::IActivationLayer *layer, ActivationType_t activationType) {
    layer->setActivationType(static_cast<nvinfer1::ActivationType>(activationType));
}

ActivationType_t activation_get_activation_type(nvinfer1::IActivationLayer *layer) {
    return static_cast<ActivationType_t>(layer->getActivationType());
}

void activation_set_alpha(nvinfer1::IActivationLayer *layer, float alpha) {
    layer->setAlpha(alpha);
}

float activation_get_alpha(nvinfer1::IActivationLayer *layer) {
    return layer->getAlpha();
}

void activation_set_beta(nvinfer1::IActivationLayer *layer, float beta) {
    layer->setBeta(beta);
}

float activation_get_beta(nvinfer1::IActivationLayer *layer) {
    return layer->getBeta();
}
