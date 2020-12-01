//
// Created by mason on 4/30/20.
//
#include <cstring>
#include <cstdlib>
#include "TRTDims.h"

nvinfer1::Dims create_dims(int nb_dims, const int* d, const DimensionType_t *dimension_types) {
    nvinfer1::Dims dims{};
    dims.nbDims = nb_dims;
    memcpy(dims.d, d, nvinfer1::Dims::MAX_DIMS * sizeof(int));
    memcpy(dims.type, dimension_types, nvinfer1::Dims::MAX_DIMS * sizeof(DimensionType));

    return dims;
}

nvinfer1::Dims2 create_dims2(int dim1, int dim2) {
    return nvinfer1::Dims2(dim1, dim2);
}

nvinfer1::DimsHW create_dimsHW(int height , int width) {
    return nvinfer1::DimsHW(height, width);
}

nvinfer1::Dims3 create_dims3(int dim1, int dim2, int dim3) {
    return nvinfer1::Dims3(dim1, dim2, dim3);
}

nvinfer1::DimsCHW create_dimsCHW(int channels, int height, int width) {
    return nvinfer1::DimsCHW(channels, height, width);
}

nvinfer1::Dims4 create_dims4(int dim1, int dim2, int dim3, int dim4) {
    return nvinfer1::Dims4(dim1, dim2, dim3, dim4);
}

nvinfer1::DimsNCHW create_dimsNCHW(int batchSize, int channel, int height, int width) {
    return nvinfer1::DimsNCHW(batchSize, channel, height, width);
}
