//
// Created by mason on 4/28/20.
//

#ifndef LIBTRT_TRTCONTEXTINTERNAL_HPP
#define LIBTRT_TRTCONTEXTINTERNAL_HPP

#include <NvInfer.h>
#include "TRTContext.h"

Context_t* create_execution_context(nvinfer1::IExecutionContext* execution_context);


#endif //LIBTRT_TRTCONTEXTINTERNAL_HPP
