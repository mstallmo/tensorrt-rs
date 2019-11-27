//
// Created by mason on 11/27/19.
//

#ifndef LIBTRT_TRTNETWORKDEFINITION_H
#define LIBTRT_TRTNETWORKDEFINITION_H

struct Network {
    void* internal_network;
};
typedef struct Network Network_t;

void destroy_network(Network_t *network);

#endif //LIBTRT_TRTNETWORKDEFINITION_H
