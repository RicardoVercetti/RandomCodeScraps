#include <stdio.h>
#include <stdlib.h>

/// `void * malloc(size_t size);`               // manual allocation by size, then free it with `free()`
/// `void * calloc (size_t n, size_t size);`
int main() {
    puts("using malloc...");

    int *pi;
    int size = 5;
    pi = (int *) malloc(size * sizeof(int));

    printf("allocated location: %p\n", &pi);

    // printf("fail code: %d\n", EXIT_FAILURE);
    if (pi == NULL) {
        printf("memory allocation failed\n");
        exit(EXIT_FAILURE);
    }

    *pi = 134123;
    
    printf("value of PI: %d\n", *pi);
    printf("size of PI: %d\n", sizeof(*pi));

    // freeup the manually allcated memory

    free(pi);


    // some error outs
    fprintf(stderr, "this is error stream...\n");
    return 0;
}