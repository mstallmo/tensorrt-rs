//
// Created by mason on 10/7/20.
//

#ifndef LIBTRT_TRTLAYER_H
#define LIBTRT_TRTLAYER_H

#ifdef __cplusplus
extern "C" {
#endif

struct Layer;
typedef struct Layer Layer_t;

const char* layer_get_name(Layer_t *layer);

#ifdef __cplusplus
};
#endif

#endif //LIBTRT_TRTLAYER_H
