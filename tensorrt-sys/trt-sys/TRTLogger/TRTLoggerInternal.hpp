//
// Created by mason on 4/11/20.
//

#ifndef LIBTRT_TRTLOGGERINTERNAL_HPP
#define LIBTRT_TRTLOGGERINTERNAL_HPP

#include <memory>
#include "TRTLogger.h"

class TRTLogger : public nvinfer1::ILogger {
public:
    explicit TRTLogger(nvinfer1::ILogger::Severity severity = nvinfer1::ILogger::Severity::kWARNING)
            : mReportableSeverity(severity) {
    }

    void log(nvinfer1::ILogger::Severity severity, const char *msg) final {
        if (severity <= mReportableSeverity)
            printf("%s\n", msg);
    }

private:
    nvinfer1::ILogger::Severity mReportableSeverity;
};

struct Logger {
    std::unique_ptr<TRTLogger> internal_logger;

    explicit Logger() {
        internal_logger = std::make_unique<TRTLogger>();
    };

    [[nodiscard]] nvinfer1::ILogger& getLogger() const;
};


#endif //LIBTRT_TRTLOGGERINTERNAL_HPP
