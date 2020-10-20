//
// Created by mason on 10/13/20.
//

#include "TRTActivationLayer.h"
#include "TRTLayerInternal.hpp"

void activation_set_activation_type(Layer_t *layer, ActivationType_t activationType) {
    auto concrete = dynamic_cast<nvinfer1::IActivationLayer*>(layer->internal_layer);
    concrete->setActivationType(static_cast<nvinfer1::ActivationType>(activationType));
}

ActivationType_t activation_get_activation_type(Layer_t *layer) {
    auto concrete = dynamic_cast<nvinfer1::IActivationLayer*>(layer->internal_layer);
    return static_cast<ActivationType_t>(concrete->getActivationType());
}

void activation_set_alpha(Layer_t *layer, float alpha) {
    auto concrete = dynamic_cast<nvinfer1::IActivationLayer*>(layer->internal_layer);
    concrete->setAlpha(alpha);
}

float activation_get_alpha(Layer_t *layer) {
    auto concrete = dynamic_cast<nvinfer1::IActivationLayer*>(layer->internal_layer);
    return concrete->getAlpha();
}

void activation_set_beta(Layer_t *layer, float beta) {
    auto concrete = dynamic_cast<nvinfer1::IActivationLayer*>(layer->internal_layer);
    concrete->setBeta(beta);
}

float activation_get_beta(Layer_t *layer) {
    auto concrete = dynamic_cast<nvinfer1::IActivationLayer*>(layer->internal_layer);
    return concrete->getBeta();
}

void activation_destroy(Layer_t *layer) {
    delete layer;
}