//
// Created by mason on 4/11/20.
//

#ifndef LIBTRT_TRTLOGGERINTERNAL_HPP
#define LIBTRT_TRTLOGGERINTERNAL_HPP

#include <memory>
#include "TRTLogger.h"

class TRTLogger : public nvinfer1::ILogger {
public:
    explicit TRTLogger(nvinfer1::ILogger::Severity severity)
            : mReportableSeverity(severity) {
    }

    void log(nvinfer1::ILogger::Severity severity, const char *msg) final {
        if (severity <= mReportableSeverity)
            printf("%s\n", msg);
    }

    void severity(nvinfer1::ILogger::Severity severity) {
        mReportableSeverity = severity;
    }

private:
    nvinfer1::ILogger::Severity mReportableSeverity;
};

struct Logger {
    std::unique_ptr<TRTLogger> internal_logger;

    explicit Logger(nvinfer1::ILogger::Severity severity) {
        internal_logger = std::make_unique<TRTLogger>(severity);
    };

    [[nodiscard]] nvinfer1::ILogger& getLogger() const;
};


#endif //LIBTRT_TRTLOGGERINTERNAL_HPP
