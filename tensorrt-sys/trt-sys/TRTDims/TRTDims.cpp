//
// Created by mason on 4/30/20.
//
#include <cstring>
#include "TRTDimsInternal.hpp"
#include <NvInfer.h>

struct Dims {
    nvinfer1::Dims internal_dims;

    explicit Dims(int nbDims, const int* d, const int* type) {
        internal_dims = nvinfer1::Dims {};
        internal_dims.nbDims = nbDims;
        memcpy(internal_dims.d, d, nvinfer1::Dims::MAX_DIMS * sizeof(int));
        memcpy(internal_dims.type, type, nvinfer1::Dims::MAX_DIMS * sizeof(int));
    }

    explicit Dims(nvinfer1::Dims dim) {
        internal_dims = dim;
    }

    explicit Dims(int dim1, int dim2) {
        internal_dims = nvinfer1::Dims2(dim1, dim2);
    }

    explicit Dims(int dim1, int dim2, int dim3) {
        internal_dims = nvinfer1::Dims3(dim1, dim2, dim3);
    }

    explicit Dims(int dim1, int dim2, int dim3, int dim4) {
        internal_dims = nvinfer1::Dims4(dim1, dim2, dim3, dim4);
    }
};

Dims_t* create_dims(int nb_dims, const int* d, const int* dimension_types) {
    return new Dims(nb_dims, d, dimension_types);
}

Dims_t* create_dims2(int dim1, int dim2) {
    return new Dims(dim1, dim2);
}

Dims_t* create_dims3(int dim1, int dim2, int dim3) {
    return new Dims(dim1, dim2, dim3);
}

Dims_t* create_dims4(int dim1, int dim2, int dim3, int dim4) {
    return new Dims(dim1, dim2, dim3, dim4);
}

nvinfer1::Dims dims_get(const Dims_t* dims) {
   return dims->internal_dims;
}

Dims_t* dims_create(nvinfer1::Dims dim) {
    return new Dims(dim);
}

void dims2_set_dimension_types(Dims_t* dims2, int type1, int type2) {
    if (dims2 == nullptr)
        return;

    dims2->internal_dims.type[0] = static_cast<nvinfer1::DimensionType>(type1);
    dims2->internal_dims.type[1] = static_cast<nvinfer1::DimensionType>(type2);
}


void dims3_set_dimension_types(Dims_t* dims3, int type1, int type2, int type3) {
    if (dims3 == nullptr)
        return;

    dims3->internal_dims.type[0] = static_cast<nvinfer1::DimensionType>(type1);
    dims3->internal_dims.type[1] = static_cast<nvinfer1::DimensionType>(type2);
    dims3->internal_dims.type[2] = static_cast<nvinfer1::DimensionType>(type3);
}

void dims4_set_dimension_types(Dims_t* dims4, int type1, int type2, int type3, int type4) {
    if (dims4 == nullptr)
        return;

    dims4->internal_dims.type[0] = static_cast<nvinfer1::DimensionType>(type1);
    dims4->internal_dims.type[1] = static_cast<nvinfer1::DimensionType>(type2);
    dims4->internal_dims.type[2] = static_cast<nvinfer1::DimensionType>(type3);
    dims4->internal_dims.type[3] = static_cast<nvinfer1::DimensionType>(type4);
}

struct DimsHW {
    nvinfer1::DimsHW internal_dimsHW;

    explicit DimsHW(int height, int width) {
        internal_dimsHW = nvinfer1::DimsHW(height, width);
    }
};

Dims_t* create_dimsHW(int height , int width) {
    return reinterpret_cast<Dims*>(new DimsHW(height, width));
}

struct DimsCHW {
    nvinfer1::DimsCHW internal_dimschw;

    explicit DimsCHW(int channels, int height, int width) {
        internal_dimschw = nvinfer1::DimsCHW(channels, height, width);
    }
};

Dims_t* create_dimsCHW(int channels, int height, int width) {
    return reinterpret_cast<Dims*>(new DimsCHW(channels, height, width));
}

struct DimsNCHW {
    nvinfer1::DimsNCHW internal_dimsnchw;

    explicit DimsNCHW(int batchSize, int channel, int height, int width) {
        internal_dimsnchw = nvinfer1::DimsNCHW(batchSize, channel, height, width);
    }
};

Dims_t* create_dimsNCHW(int batchSize, int channel, int height, int width) {
    return reinterpret_cast<Dims*>(new DimsNCHW(batchSize, channel, height, width));
}

void destroy_dims(Dims_t* dims) {
    if(dims == nullptr)
        return;

    delete dims;
}
