//
// Created by mason on 8/25/19.
//
#include <stdio.h>
#include <stdlib.h>
#include "tensorrt_api.h"

int main() {
    char tensorrt_version[6];
    get_tensorrt_version(tensorrt_version);
    printf("Your TensorRT version is %s\n", tensorrt_version);

    Logger_t* logger = create_logger();
    log_error(logger, "This is a logging test\n");

    Runtime_t* runtime = create_infer_runtime(logger);
    FILE *fp = fopen("../../resnet34-unet-Aug25-07-25-16-best.engine", "rb");

    if (fp == NULL)
        printf("Could not open file\n");

    if (fseek(fp, 0, SEEK_END) !=  0)
        printf("Could not get model size\n");

    size_t size = ftell(fp);
    void* data = malloc(size);
    fseek(fp, 0, SEEK_SET);
    size_t n = fread(data, size, 1, fp);

    Engine_t* engine = deserialize_cuda_engine(runtime, data, size);

    const char* binding_name = get_binding_name(engine, 0);
    printf("Model binding name at index %d is %s", 0, binding_name);

    destroy_cuda_engine(engine);
    destroy_infer_runtime(runtime);
    delete_logger(logger);
    return 0;
}