#include <stdio.h>

int main() {
    puts("dynamic memory allocation intro...");
    // available methods are malloc() , realloc() , calloc() and free()
    // sizeof() returns a data type called size_t whose size is dynamic for a system, typically 4bytes

    // int *a = (void *)NULL;

    // printf("size: %d byte(s)\n", sizeof(NULL));
    // printf("addr: %d\n", a);
    // printf("nullval: %b\n", NULL);

    int *ptr_a;

    int val_a = 10;
    ptr_a = &val_a;
    printf("val a: %d\n", val_a);
    printf("ptr_a: %p\n", ptr_a);
    printf("addr val_a: %p\n", &val_a);

    // unsafe deref
    // printf("unsafe deref val_a: %d\n", *val_a);

    // after assigning the value:
    // printf("in ptr after assigning: %d\n", *ptr_a);

    return 0;
}