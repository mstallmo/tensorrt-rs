cmake_minimum_required(VERSION 3.10)
project(LibTRT LANGUAGES CXX CUDA)

if(DEFINED TRT7)
    message(STATUS "TRT7 is defined")
    add_compile_definitions(TRT7)
elseif(DEFINED TRT6)
    message(STATUS "TRT6 is defined")
    add_compile_definitions(TRT6)
endif()

set(CMAKE_CXX_STANDARD 17)

set(CMAKE_CXX_FLAGS "-fPIC -O3 -Wall -Wextra -Werror -Wno-unknown-pragmas -Wno-deprecated -Wno-deprecated-declarations")

file(GLOB source_files
        "TRTLogger/*.cpp"
        "TRTRuntime/*cpp"
        "TRTCudaEngine/*.cpp"
        "TRTContext/*.cpp"
        "TRTUffParser/*.cpp"
        "TRTOnnxParser/*.cpp"
        "TRTDims/*.cpp"
        "TRTBuilder/*.cpp"
        "TRTNetworkDefinition/*.cpp"
        "TRTHostMemory/*.cpp"
        "TRTLayer/*.cpp"
)

add_library(trt-sys STATIC ${source_files})
include_directories(${CMAKE_CUDA_TOOLKIT_INCLUDE_DIRECTORIES})

if(DEFINED ENV{TRT_INSTALL_DIR})
    target_include_directories(trt-sys PRIVATE $ENV{TRT_INSTALL_DIR}/include)
endif()

install(TARGETS trt-sys DESTINATION .)
