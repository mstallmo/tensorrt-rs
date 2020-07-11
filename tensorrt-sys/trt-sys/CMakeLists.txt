cmake_minimum_required(VERSION 3.10)
project(LibTRT LANGUAGES CXX CUDA)

set(CMAKE_CXX_STANDARD 17)

find_package(CUDA 11.0 REQUIRED)

#set(CMAKE_CXX_FLAGS "-O3 -Wall -Wextra -Werror -Wno-unknown-pragmas")

file(GLOB source_files
        "TRTLogger/*.h"
        "TRTLogger/*.cpp"
        "TRTRuntime/*.h"
        "TRTRuntime/*cpp"
        "TRTCudaEngine/*.h"
        "TRTCudaEngine/*.cpp"
        "TRTContext/*.h"
        "TRTContext/*.cpp"
        "TRTUffParser/*.h"
        "TRTUffParser/*.cpp"
        "TRTDims/*.h"
        "TRTDims/*.cpp"
        "TRTBuilder/*.h"
        "TRTBuilder/*.cpp"
        "TRTNetworkDefinition/*.h"
        "TRTNetworkDefinition/*.cpp"
        "TRTHostMemory/*.h"
        "TRTHostMemory/*.cpp"
        "*.h"
        )

find_library(NVINFER nvinfer)

find_library(NVPARSERS nvparsers)

#add_executable(trt-sys-test ${source_files} main.cpp)
add_library(trt-sys SHARED ${source_files})
target_link_libraries(trt-sys PRIVATE ${NVINFER} ${NVPARSERS} ${CUDA_LIBRARIES})
#target_link_libraries(trt-sys-test PRIVATE ${NVINFER} ${NVPARSERS} ${CUDA_LIBRARIES})
include_directories(${CMAKE_CUDA_TOOLKIT_INCLUDE_DIRECTORIES})
include_directories("D://projects//TensorRT-7.1.3.4//include")

install(TARGETS trt-sys DESTINATION .)