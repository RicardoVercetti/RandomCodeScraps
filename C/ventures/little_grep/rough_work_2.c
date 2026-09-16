#include <stdio.h>

// Notes:
// 1. when using indexes or `+` operations in arrays, it adds the base pointer address by size of each
//    i.e: for int a[4], and size of an int is 4 bytes. and *(a+1) - the second element a[2] is actually 4 bytes after the address of first one.    

void f(int* a, size_t size) {
    printf("size insize function: %zu\n", sizeof(a));
    printf("address: %p\n", a);
    printf("in hex: %x\n", a);

    for (size_t i=0; i<size; i++) {
        printf("item[%d]: %d\n", i, a[i]);
    }
}

int main() {
    printf("here goes nothing...\n");

    int a[5] = {1, 2, 3, 4, 5};

    printf("first value: %d\n", a[0]);
    printf("address: %p\n", a);
    printf("first item address: %x\n", &a[0]);
    printf("last value: %d\n", a[4]);

    printf("size_t: %d\n", sizeof(size_t));
    printf("int: %d\n", sizeof(int));
    printf("bool: %d\n", sizeof(bool));
    printf("char: %d\n", sizeof(char));
    printf("double: %d\n", sizeof(double));
    printf("float: %d\n", sizeof(float));

    printf("size of a as total: %d\n", sizeof(a));
    printf("size of one in a: %d\n", sizeof(a[0]));

    f(a, sizeof(a)/sizeof(a[0]));

    return 0;
}