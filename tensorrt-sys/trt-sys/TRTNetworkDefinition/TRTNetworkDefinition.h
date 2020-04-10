//
// Created by mason on 11/27/19.
//

#ifndef LIBTRT_TRTNETWORKDEFINITION_H
#define LIBTRT_TRTNETWORKDEFINITION_H

#ifdef __cplusplus
extern "C" {
#endif

struct Network;
typedef struct Network Network_t;

void destroy_network(Network_t *network);

#ifdef __cplusplus
};
#endif

#endif //LIBTRT_TRTNETWORKDEFINITION_H
