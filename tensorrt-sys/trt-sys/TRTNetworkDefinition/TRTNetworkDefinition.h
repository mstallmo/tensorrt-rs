//
// Created by mason on 11/27/19.
//

#ifndef LIBTRT_TRTNETWORKDEFINITION_H
#define LIBTRT_TRTNETWORKDEFINITION_H

#include "../DLLExport.h"

#ifdef __cplusplus
extern "C" {
#endif

struct Network;
typedef struct Network Network_t;

DllExport void destroy_network(Network_t *network);

#ifdef __cplusplus
};
#endif

#endif //LIBTRT_TRTNETWORKDEFINITION_H
