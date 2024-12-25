#include <stdio.h>
#include <stdlib.h>

struct MyStruct {
    int a;
    float b;
};

void* malloc_and_access(int a, float b) {
    struct MyStruct* ptr = (struct MyStruct*)malloc(sizeof(struct MyStruct) + 1);

    if (ptr == NULL) {
        printf("malloc fail\n");
        return NULL;
    }

    ptr->a = a;
    ptr->b = b;

    return ptr;
}

void free_memory(void* ptr) {
    free(ptr);
}
