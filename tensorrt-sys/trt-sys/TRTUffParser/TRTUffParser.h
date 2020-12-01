//
// Created by mason on 11/22/19.
//

#ifndef LIBTRT_TRTUFFPARSER_H
#define LIBTRT_TRTUFFPARSER_H

#include <NvInfer.h>
#include <stdbool.h>

#include "../TRTNetworkDefinition/TRTNetworkDefinition.h"

#ifdef __cplusplus
extern "C" {
#endif

struct UffParser;
typedef struct UffParser UffParser_t;

UffParser_t* uffparser_create_uff_parser();
void uffparser_destroy_uff_parser(UffParser_t* uff_parser);

bool uffparser_register_input(const UffParser_t* uff_parser, const char* input_name, nvinfer1::Dims dims, int input_order);
bool uffparser_register_output(const UffParser_t* uff_parser, const char* output_name);

bool uffparser_parse(const UffParser_t* uff_parser, const char* file, nvinfer1::INetworkDefinition *network);

#ifdef __cplusplus
};
#endif

#endif //LIBTRT_TRTUFFPARSER_H
