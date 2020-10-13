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

enum LayerType {
    kCONVOLUTION = 0,
    kFULLY_CONNECTED = 1,
    kACTIVATION  = 2,
    kPOOLING = 3,
    kLRN = 4,
    kSCALE = 5,
    kSOFTMAX = 6,
    kDECONVOLUTION = 7,
    kCONCATENATION = 8,
    kELEMENTWISE = 9,
    kPLUGIN = 10,
    kRNN = 11,
    kUNARY = 12,
    kPADDING = 13,
    kSHUFFLE = 14,
    kREDUCE = 15,
    kTOPK = 16,
    kGATHER = 17,
    kMATRIX_MULTIPLY = 18,
    kRAGGED_SOFTMAX = 19,
    kCONSTANT = 20,
    kRNN_V2 = 21,
    kIDENTITY = 22,
    kPLUGIN_V2 = 23,
    kSLICE = 24,
};
typedef enum LayerType LayerType_t;

enum TensorLocation {
    kDEVICE = 0,
    kHOST = 1,
};
typedef enum TensorLocation TensorLocation_t;

#ifdef __cplusplus
};
#endif

#endif //LIBTRT_TRTENUMS_H
