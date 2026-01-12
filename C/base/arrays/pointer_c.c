#include <stdio.h>

int main() {

    puts("pointer in C...");

    int x = 10;
    int *px = &x;

    printf("data of *px = %d\n", *px);

    *px = 20;

    printf("x = %d\n", x);
    printf("*px = %d\n", *px);

    printf("addr of x: %p\n", &x);
    printf("addr of *px: %p\n", px);
    printf("addr fo pointer: %p\n", &px);

    return 0;
}

// 7fff89985e6c