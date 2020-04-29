//
// Created by mason on 4/28/20.
//

#ifndef LIBTRT_TRTCUDAENGINEINTERNAL_HPP
#define LIBTRT_TRTCUDAENGINEINTERNAL_HPP

#include <NvInfer.h>

#include "TRTCudaEngine.h"

Engine_t* create_engine(nvinfer1::ICudaEngine* engine);

#endif //LIBTRT_TRTCUDAENGINEINTERNAL_HPP
