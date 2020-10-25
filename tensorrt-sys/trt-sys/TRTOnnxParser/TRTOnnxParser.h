#ifndef LIBTRT_TRTONNXPARSER_H
#define LIBTRT_TRTONNXPARSER_H

#include <NvInfer.h>
#include "../TRTDims/TRTDims.h"
#include "../TRTNetworkDefinition/TRTNetworkDefinition.h"
#include "../TRTLogger/TRTLogger.h"

#ifdef __cplusplus
extern "C" {
#endif

struct OnnxParser;
typedef struct OnnxParser OnnxParser_t;

OnnxParser_t* onnxparser_create_parser(nvinfer1::INetworkDefinition *network, Logger_t *logger);
void onnxparser_destroy_parser(OnnxParser_t* onnx_parser);

bool onnxparser_parse_from_file(const OnnxParser_t* onnx_parser, const char* file, int verbosity);

#ifdef __cplusplus
};
#endif

#endif //LIBTRT_TRTONNXPARSER_H
