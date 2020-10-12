//
// Created by mason on 10/10/20.
//

#ifndef LIBTRT_TRTENUMS_H
#define LIBTRT_TRTENUMS_H

#ifdef __cplusplus
extern "C" {
#endif

enum DataType {
    kFLOAT = 0,
    kHALF = 1,
    kINT8 = 2,
    kINT32 = 3,
};
typedef enum DataType DataType_t;

enum DeviceType {
    kGPU = 0,
    kDLA = 1,
};
typedef enum DeviceType DeviceType_t;

enum DimensionType {
    kSPATIAL = 0,
    kCHANNEL = 1,
    kINDEX = 2,
    kSEQUENCE = 3
};
typedef enum DimensionType DimensionType_t;

enum EngineCapabiliy {
    kDEFAULT = 0,
    kSAFE_GPU = 1,
    kSAFE_DLA = 2,
};
typedef enum EngineCapabiliy EngineCapabiliy_t;

enum TensorLocation {
    kDEVICE = 0,
    kHOST = 1,
};
typedef enum TensorLocation TensorLocation_t;

#ifdef __cplusplus
};
#endif

#endif //LIBTRT_TRTENUMS_H
