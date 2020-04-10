//
// Created by mason on 11/22/19.
//
#include <cstring>
#include <memory>

#include "NvUffParser.h"

#include "TRTUffParser.h"
#include "../TRTNetworkDefinition/TRTNetworkDefinitionInternal.hpp"

struct UffParser {
    nvuffparser::IUffParser *internal_uffParser;

    explicit UffParser(nvuffparser::IUffParser* uffParser) : internal_uffParser(uffParser) {};

    ~UffParser() {
        internal_uffParser->destroy();
    }
};

UffParser_t *uffparser_create_uff_parser() {
    return new UffParser(nvuffparser::createUffParser());
}

void uffparser_destroy_uff_parser(UffParser_t *uff_parser) {
    if (uff_parser == nullptr)
        return;

    delete uff_parser;
}

bool uffparser_register_input(const UffParser_t *uff_parser, const char *input_name, const struct Dims input_dims) {
    if (uff_parser == nullptr)
        return false;

    nvuffparser::IUffParser *uffParser = uff_parser->internal_uffParser;

    nvinfer1::Dims nvDims = {};
    nvDims.nbDims = input_dims.nbDims;
    memcpy(nvDims.d, input_dims.d, input_dims.nbDims * sizeof(int));
    memcpy(nvDims.type, input_dims.type, input_dims.nbDims * sizeof(int));

    return uffParser->registerInput(input_name, nvDims, nvuffparser::UffInputOrder::kNCHW);
}

bool uffparser_register_output(const UffParser_t *uff_parser, const char *output_name) {
    if (uff_parser == nullptr)
        return false;

    nvuffparser::IUffParser *uffParser = uff_parser->internal_uffParser;

    return uffParser->registerOutput(output_name);
}

bool uffparser_parse(const UffParser_t *uff_parser, const char *file, const Network_t *network) {
    if (uff_parser == nullptr)
        return false;

    nvuffparser::IUffParser *uffParser = uff_parser->internal_uffParser;
    auto networkDefinition = getNetworkDefinition(network);

    return uffParser->parse(file, *networkDefinition, nvinfer1::DataType::kFLOAT);
}