//
// Created by mason on 8/25/19.
//

#ifndef TENSRORT_SYS_TRTLOGGER_H
#define TENSRORT_SYS_TRTLOGGER_H

#include "../DLLExport.h"

#ifdef __cplusplus
extern "C" {
#endif

struct Logger;
typedef struct Logger Logger_t;

DllExport void get_tensorrt_version(char* string);

DllExport Logger_t* create_logger();
DllExport void log_error(Logger_t* logger, char* err);
DllExport void delete_logger(Logger_t* logger);


#ifdef __cplusplus
};
#endif

#endif //TENSRORT_SYS_TRTLOGGER_H
