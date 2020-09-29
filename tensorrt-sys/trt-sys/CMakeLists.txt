cmake_minimum_required(VERSION 3.10)
project(LibTRT LANGUAGES CXX CUDA)

if(${TRT_VERSION} MATCHES "5.1.5")
    message(STATUS "TRT version is ${TRT_VERSION}")
    add_definitions(-DTRT_VERSION="${TRT_VERSION}")
endif()

set(CMAKE_CXX_STANDARD 17)

set(CMAKE_CXX_FLAGS "-fPIC -O3 -Wall -Wextra -Werror -Wno-unknown-pragmas")

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

add_library(trt-sys STATIC ${source_files})
include_directories(${CMAKE_CUDA_TOOLKIT_INCLUDE_DIRECTORIES})

install(TARGETS trt-sys DESTINATION .)