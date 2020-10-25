//
// Created by mason on 11/22/19.
//
#include <cstring>
#include <memory>

#include "NvUffParser.h"

#include "TRTUffParser.h"
#include "../TRTDims/TRTDimsInternal.hpp"
#include "../TRTUtils.hpp"

struct UffParser {
    using IUffParserPtr = std::unique_ptr<nvuffparser::IUffParser, TRTDeleter<nvuffparser::IUffParser>>;
    IUffParserPtr internal_uffParser;

    explicit UffParser(nvuffparser::IUffParser *uffParser) : internal_uffParser(uffParser) {};
};

UffParser_t *uffparser_create_uff_parser() {
    return new UffParser(nvuffparser::createUffParser());
}

void uffparser_destroy_uff_parser(UffParser_t *uff_parser) {
    if (uff_parser == nullptr)
        return;

    delete uff_parser;
}

bool uffparser_register_input(const UffParser_t *uff_parser, const char *input_name, const Dims_t* dims, int input_order) {
    if (uff_parser == nullptr || input_name == nullptr || dims == nullptr)
        return false;

    auto inputOrder = static_cast<nvuffparser::UffInputOrder>(input_order);
    return uff_parser->internal_uffParser->registerInput(input_name, dims_get(dims), inputOrder);
}

bool uffparser_register_output(const UffParser_t *uff_parser, const char *output_name) {
    if (uff_parser == nullptr || output_name == nullptr)
        return false;

    return uff_parser->internal_uffParser->registerOutput(output_name);
}

bool uffparser_parse(const UffParser_t *uff_parser, const char *file, nvinfer1::INetworkDefinition *network) {
    if (uff_parser == nullptr || file == nullptr || network == nullptr)
        return false;

    return uff_parser->internal_uffParser->parse(file, *network, nvinfer1::DataType::kFLOAT);
}