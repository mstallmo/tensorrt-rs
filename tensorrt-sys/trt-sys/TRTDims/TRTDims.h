//
// Created by mason on 11/22/19.
//

#ifndef LIBTRT_TRTDIMS_H
#define LIBTRT_TRTDIMS_H

#ifdef __cplusplus
extern "C" {
#endif

enum DimensionType {
    kSPATIAL = 0,
    kCHANNEL = 1,
    kINDEX = 2,
    kSEQUENCE = 3
};
typedef enum DimensionType DimensionType_t;

struct Dims {
    int nbDims;
    int d[8];
    DimensionType_t type[8];
};
typedef struct Dims Dims_t;

Dims_t* create_dims(int nb_dims, const int* d, const DimensionType_t *dimension_types);
Dims_t* create_dims2(int dim1, int dim2);
Dims_t* create_dimsHW(int height, int width);
void dims2_set_dimension_types(Dims_t* dims2, DimensionType_t type1, DimensionType_t type2);
Dims_t* create_dims3(int dim1, int dim2, int dim3);
Dims_t* create_dimsCHW(int channel, int height, int width);
void dims3_set_dimension_types(Dims_t* dims3, DimensionType_t type1, DimensionType_t type2, DimensionType_t type3);
Dims_t* create_dims4(int dim1, int dim2, int dim3, int dim4);
Dims_t* create_dimsNCHW(int index, int channel, int height, int width);
void dims4_set_dimension_types(Dims_t* dims4, DimensionType_t type1, DimensionType_t type2, DimensionType_t type3, DimensionType_t type4);

void destroy_dims(Dims_t* dims);


#ifdef __cplusplus
};
#endif

#endif //LIBTRT_TRTDIMS_H
