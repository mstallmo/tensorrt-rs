//
// Created by mason on 10/7/20.
//

#ifndef LIBTRT_TRTLAYERINTERNAL_HPP
#define LIBTRT_TRTLAYERINTERNAL_HPP

#include <NvInfer.h>
#include "TRTLayer.h"

struct Layer {
    nvinfer1::ILayer* internal_layer;

    explicit Layer(nvinfer1::ILayer* layer) : internal_layer(layer){}
};


#endif //LIBTRT_TRTLAYERINTERNAL_HPP
