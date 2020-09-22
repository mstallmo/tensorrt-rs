//
// Created by mason on 4/30/20.
//
#include <cstring>
#include <cstdlib>
#include "TRTDimsInternal.hpp"
#include <NvInfer.h>

//struct Dims {
//    nvinfer1::Dims internal_dims;
//
//    explicit Dims(int nbDims, const int* d, const int* type) {
//        internal_dims = nvinfer1::Dims {};
//        internal_dims.nbDims = nbDims;
//        memcpy(internal_dims.d, d, nvinfer1::Dims::MAX_DIMS * sizeof(int));
//        memcpy(internal_dims.type, type, nvinfer1::Dims::MAX_DIMS * sizeof(int));
//    }
//
//    explicit Dims(int dim1, int dim2) {
//        internal_dims = nvinfer1::Dims2(dim1, dim2);
//    }
//
//    explicit Dims(int dim1, int dim2, int dim3) {
//        internal_dims = nvinfer1::Dims3(dim1, dim2, dim3);
//    }
//
//    explicit Dims(int dim1, int dim2, int dim3, int dim4) {
//        internal_dims = nvinfer1::Dims4(dim1, dim2, dim3, dim4);
//    }
//};

Dims_t* create_dims(int nb_dims, const int* d, const DimensionType_t *dimension_types) {
    auto dims = static_cast<Dims_t*>(malloc(sizeof(Dims_t)));
    dims->nbDims = nb_dims;
    memcpy(dims->d, d, nvinfer1::Dims::MAX_DIMS * sizeof(int));
    memcpy(dims->type, dimension_types, nvinfer1::Dims::MAX_DIMS * sizeof(DimensionType));

    return dims;
}

Dims_t* create_dims2(int dim1, int dim2) {
    auto dims = static_cast<Dims_t*>(malloc(sizeof(Dims_t)));
    dims->nbDims = 2;
    dims->d[0] = dim1;
    dims->d[1] = dim2;

    return dims;
}

Dims_t* create_dimsHW(int height , int width) {
    auto dims = static_cast<Dims_t*>(malloc(sizeof(Dims_t)));
    dims->nbDims = 2;
    dims->d[0] = height;
    dims->d[1] = width;
    dims->type[0] = kSPATIAL;
    dims->type[1] = kSPATIAL;

    return dims;
}

Dims_t* create_dims3(int dim1, int dim2, int dim3) {
    auto dims = static_cast<Dims_t*>(malloc(sizeof(Dims_t)));
    dims->nbDims = 3;
    dims->d[0] = dim1;
    dims->d[1] = dim2;
    dims->d[2] = dim3;

    return dims;
}
Dims_t* create_dimsCHW(int channels, int height, int width) {
    auto dims = static_cast<Dims_t*>(malloc(sizeof(Dims_t)));
    dims->nbDims = 3;
    dims->d[0] = channels;
    dims->d[1] = height;
    dims->d[2] = width;
    dims->type[0] = kCHANNEL;
    dims->type[1] = kSPATIAL;
    dims->type[2] = kSPATIAL;

    return dims;
}

Dims_t* create_dims4(int dim1, int dim2, int dim3, int dim4) {
    auto dims = static_cast<Dims_t*>(malloc(sizeof(Dims_t)));
    dims->nbDims = 4;
    dims->d[0] = dim1;
    dims->d[1] = dim2;
    dims->d[2] = dim3;
    dims->d[3] = dim4;

    return dims;
}

Dims_t* create_dimsNCHW(int batchSize, int channel, int height, int width) {
    auto dims = static_cast<Dims_t*>(malloc(sizeof(Dims_t)));
    dims->nbDims = 4;
    dims->d[0] = batchSize;
    dims->d[1] = channel;
    dims->d[2] = height;
    dims->d[3] = width;
    dims->type[0] = kINDEX;
    dims->type[1] = kCHANNEL;
    dims->type[2] = kSPATIAL;
    dims->type[2] = kSPATIAL;

    return dims;
}

nvinfer1::Dims dims_get(const Dims_t* dims) {
    nvinfer1::Dims nvDim{};
    nvDim.nbDims = dims->nbDims;
    memcpy(nvDim.d, dims->d, nvinfer1::Dims::MAX_DIMS * sizeof(int));
    memcpy(nvDim.type, dims->type, nvinfer1::Dims::MAX_DIMS * sizeof(int));
    return nvDim;
}

void dims2_set_dimension_types(Dims_t* dims2, DimensionType_t type1, DimensionType_t type2) {
    if (dims2 == nullptr)
        return;

    dims2->type[0] = type1;
    dims2->type[1] = type2;
}

void dims3_set_dimension_types(Dims_t* dims3, DimensionType_t type1, DimensionType_t type2, DimensionType_t type3) {
    if (dims3 == nullptr)
        return;

    dims3->type[0] = type1;
    dims3->type[1] = type2;
    dims3->type[2] = type3;
}

void dims4_set_dimension_types(Dims_t* dims4, DimensionType_t type1, DimensionType_t type2, DimensionType_t type3, DimensionType_t type4) {
    if (dims4 == nullptr)
        return;

    dims4->type[0] = type1;
    dims4->type[1] = type2;
    dims4->type[2] = type3;
    dims4->type[3] = type4;
}

void destroy_dims(Dims_t* dims) {
    if(dims == nullptr)
        return;

    free(dims);
}
