//
// Created by mason on 11/22/19.
//
#include <cstdlib>
#include <cstring>

#include "NvUffParser.h"

#include "TRTUffParser.h"

struct UffParser {
    void* internal_uffParser;
};

UffParser_t* uffparser_create_uff_parser() {
    UffParser_t* u;

    u = (typeof(u))malloc(sizeof(u));
    u->internal_uffParser = nvuffparser::createUffParser();

    return u;
}

void uffparser_destroy_uff_parser(UffParser_t* uff_parser) {
    if (uff_parser == nullptr)
        return;

    auto uffParser = static_cast<nvuffparser::IUffParser*>(uff_parser->internal_uffParser);
    uffParser->destroy();

    free(uff_parser);
}

bool uffparser_register_input(const UffParser_t* uff_parser, const char* input_name, const struct Dims input_dims) {
    if (uff_parser == nullptr)
        return false;

    auto uffParser = static_cast<nvuffparser::IUffParser*>(uff_parser->internal_uffParser);

    nvinfer1::Dims nvDims = {};
    nvDims.nbDims = input_dims.nbDims;
    memcpy(nvDims.d, input_dims.d, input_dims.nbDims * sizeof(int));
    memcpy(nvDims.type, input_dims.type, input_dims.nbDims * sizeof(int));

    return uffParser->registerInput(input_name, nvDims, nvuffparser::UffInputOrder::kNCHW);
}

bool uffparser_register_output(const UffParser_t* uff_parser, const char* output_name) {
    if (uff_parser == nullptr)
        return false;

    auto uffParser = static_cast<nvuffparser::IUffParser *>(uff_parser->internal_uffParser);

    return uffParser->registerOutput(output_name);
}

bool uffparser_parse(const UffParser_t* uff_parser, const char* file, const Network_t* network)
{
    if (uff_parser == nullptr)
        return false;

    auto uffParser = static_cast<nvuffparser::IUffParser*>(uff_parser->internal_uffParser);
    auto networkDefinition = static_cast<nvinfer1::INetworkDefinition*>(network->internal_network);

    return uffParser->parse(file, *networkDefinition, nvinfer1::DataType::kFLOAT);
}