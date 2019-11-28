//
// Created by mason on 8/25/19.
//
#include <cstdlib>
#include <cstdio>
#include "NvInfer.h"
#include "TRTLogger.h"


class TRTLogger : nvinfer1::ILogger
{
public:
    explicit TRTLogger(nvinfer1::ILogger::Severity severity = nvinfer1::ILogger::Severity::kWARNING)
            : mReportableSeverity(severity)
    {
    }


    void log(nvinfer1::ILogger::Severity severity, const char* msg) override
    {
        if (severity <= mReportableSeverity)
            printf("%s\n", msg);
    }
private:
    nvinfer1::ILogger::Severity mReportableSeverity;
};

void get_tensorrt_version(char* string) {
    sprintf(string, "%d.%d.%d", NV_TENSORRT_MAJOR, NV_TENSORRT_MINOR, NV_TENSORRT_PATCH);
}

Logger_t* create_logger() {
    Logger_t *l;
    TRTLogger *obj;

    l = (typeof(l))malloc(sizeof(*l));
    obj = new TRTLogger();
    l->internal_logger = obj;

    return l;
}

void delete_logger(Logger_t* logger) {
    if (logger == nullptr)
        return;
    delete static_cast<TRTLogger *>(logger->internal_logger);
    free(logger);
}


void log_error(Logger_t* logger, char* err) {
    if (logger == nullptr)
        return;
    auto l = static_cast<TRTLogger *>(logger->internal_logger);
    l->log(nvinfer1::ILogger::Severity::kWARNING, err);
}