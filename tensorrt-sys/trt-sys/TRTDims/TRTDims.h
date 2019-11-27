//
// Created by mason on 11/22/19.
//

#ifndef LIBTRT_TRTDIMS_H
#define LIBTRT_TRTDIMS_H

#ifdef __cplusplus
extern "C" {
#endif

struct Dims {
    int nbDims;
    int* d;
    int* type;
};

#ifdef __cplusplus
};
#endif

#endif //LIBTRT_TRTDIMS_H
