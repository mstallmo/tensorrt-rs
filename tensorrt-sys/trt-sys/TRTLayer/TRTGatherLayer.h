//
// Created by mason on 10/12/20.
//

#ifndef LIBTRT_TRTGATHERLAYER_H
#define LIBTRT_TRTGATHERLAYER_H

#include <stdint.h>
#include "TRTLayer.h"

#ifdef __cplusplus
extern "C" {
#endif

int32_t gather_layer_get_gather_axis(Layer_t *layer);

void gather_layer_set_gather_axis(Layer_t *layer, int32_t axis);

void gather_layer_destroy(Layer_t *layer);

#ifdef __cplusplus
};
#endif

#endif //LIBTRT_TRTGATHERLAYER_H
