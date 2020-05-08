//
// Created by mason on 11/22/19.
//

#ifndef LIBTRT_TRTDIMS_H
#define LIBTRT_TRTDIMS_H

#ifdef __cplusplus
extern "C" {
#endif

struct Dims;
typedef struct Dims Dims_t;
struct Dims2;
typedef struct Dims2 Dims2_t;
struct Dims3;
typedef struct Dims3 Dims3_t;
struct Dims4;
typedef struct Dims4 Dims4_t;
struct DimsHW;
typedef struct DimsHW DimsHW_t;
struct DimsCHW;
typedef struct DimsCHW DimsCHW_t;
struct DimsNCHW;
typedef struct DimsNCHW DimsNCHW_t;

Dims_t* create_dims(int nb_dims, const int* d, const int* dimension_types);
Dims_t* create_dims2(int dim1, int dim2);
Dims_t* create_dims3(int dim1, int dim2, int dim3);
Dims_t* create_dims4(int dim1, int dim2, int dim3, int dim4);
Dims_t* create_dimsHW(int height, int width);
Dims_t* create_dimsCHW(int channel, int height, int width);
Dims_t* create_dimsNCHW(int index, int channel, int height, int width);

void destroy_dims(Dims_t* dims);

void dims2_set_dimension_types(Dims_t* dims2, int type1, int type2);
void dims3_set_dimension_types(Dims_t* dims3, int type1, int type2, int type3);
void dims4_set_dimension_types(Dims_t* dims4, int type1, int type2, int type3, int type4);


#ifdef __cplusplus
};
#endif

#endif //LIBTRT_TRTDIMS_H
