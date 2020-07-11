//
// Created by mason on 11/22/19.
//

#ifndef LIBTRT_TRTUFFPARSER_H
#define LIBTRT_TRTUFFPARSER_H

#include <stdbool.h>

#include "../DLLExport.h"
#include "../TRTDims/TRTDims.h"
#include "../TRTNetworkDefinition/TRTNetworkDefinition.h"

#ifdef __cplusplus
extern "C" {
#endif

struct UffParser;
typedef struct UffParser UffParser_t;

DllExport UffParser_t* uffparser_create_uff_parser();
DllExport void uffparser_destroy_uff_parser(UffParser_t* uff_parser);

DllExport bool uffparser_register_input(const UffParser_t* uff_parser, const char* input_name, const Dims_t* dims, int input_order);
DllExport bool uffparser_register_output(const UffParser_t* uff_parser, const char* output_name);

DllExport bool uffparser_parse(const UffParser_t* uff_parser, const char* file, const Network_t *network);

#ifdef __cplusplus
};
#endif

#endif //LIBTRT_TRTUFFPARSER_H
