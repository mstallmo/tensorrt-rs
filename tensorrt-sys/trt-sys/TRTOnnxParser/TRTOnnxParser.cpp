#include <cstring>
#include <memory>

#include "NvOnnxParser.h"

#include "TRTOnnxParser.h"
#include "../TRTDims/TRTDimsInternal.hpp"
#include "../TRTLogger/TRTLoggerInternal.hpp"
#include "../TRTUtils.hpp"

struct OnnxParser {
    using IOnnxParserPtr = std::unique_ptr<nvonnxparser::IParser, TRTDeleter<nvonnxparser::IParser>>;
    IOnnxParserPtr internal_onnxParser;

    explicit OnnxParser(nvonnxparser::IParser *onnxParser) : internal_onnxParser(onnxParser) {};
};

OnnxParser_t *onnxparser_create_parser(nvinfer1::INetworkDefinition *network, Logger_t *logger) {
    return new OnnxParser(nvonnxparser::createParser(*network, logger->getLogger()));
}

void onnxparser_destroy_parser(OnnxParser_t *onnx_parser) {
    if (onnx_parser == nullptr)
        return;

    delete onnx_parser;
}

bool onnxparser_parse_from_file(const OnnxParser_t *onnx_parser, const char *file, int verbosity) {
    if (onnx_parser == nullptr || file == nullptr)
        return false;

    return onnx_parser->internal_onnxParser->parseFromFile(file, verbosity);
}
