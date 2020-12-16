//
// Created by mason on 10/12/20.
//

#ifndef LIBTRT_TRTGATHERLAYER_H
#define LIBTRT_TRTGATHERLAYER_H

#include <stdint.h>
#include "TRTLayer.h"

int32_t gather_layer_get_gather_axis(nvinfer1::IGatherLayer *layer);
void gather_layer_set_gather_axis(nvinfer1::IGatherLayer *layer, int32_t axis);

#endif //LIBTRT_TRTGATHERLAYER_H
