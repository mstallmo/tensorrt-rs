//
// Created by mason on 10/12/20.
//

#ifndef LIBTRT_TRTELEMENTWISELAYER_H
#define LIBTRT_TRTELEMENTWISELAYER_H

#include "TRTLayer.h"
#include "../TRTEnums.h"

#ifdef __cplusplus
extern "C" {
#endif

void elementwise_set_operation(Layer_t *element_wise_layer, ElementWiseOperation_t type);

ElementWiseOperation_t elementwise_get_operation(Layer_t *element_wise_layer);

void elementwise_destroy(Layer_t *element_wise_layer);

#ifdef __cplusplus
};
#endif

#endif //LIBTRT_TRTELEMENTWISELAYER_H
