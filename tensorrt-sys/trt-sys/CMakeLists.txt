cmake_minimum_required(VERSION 3.10)
project(LibTRT LANGUAGES CXX CUDA)

if(${TRT_VERSION} MATCHES "5.1.5")
    message(STATUS "TRT version is ${TRT_VERSION}")
    add_definitions(-DTRT_VERSION="${TRT_VERSION}")
endif()

set(CMAKE_CXX_STANDARD 17)

set(CMAKE_CXX_FLAGS "-O3 -Wall -Wextra -Werror -Wno-unknown-pragmas")

file(GLOB source_files
        "TRTLogger/*.cpp"
        "TRTRuntime/*cpp"
        "TRTCudaEngine/*.cpp"
        "TRTContext/*.cpp"
        "TRTUffParser/*.cpp"
        "TRTDims/*.cpp"
        "TRTBuilder/*.cpp"
        "TRTNetworkDefinition/*.cpp"
        "TRTHostMemory/*.cpp"
        )

find_library(CUDART_LIBRARY cudart ${CMAKE_CUDA_IMPLICIT_LINK_DIRECTORIES})

add_library(trt-sys STATIC ${source_files})
target_link_libraries(trt-sys PRIVATE nvinfer ${CUDART_LIBRARY})
include_directories(${CMAKE_CUDA_TOOLKIT_INCLUDE_DIRECTORIES})

install(TARGETS trt-sys DESTINATION .)