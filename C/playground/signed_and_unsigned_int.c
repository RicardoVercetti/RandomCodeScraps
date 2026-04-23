#include <stdio.h>

int main() {
    int a = 255666;
    unsigned int b = 255666;

    printf("a: %d\n", a);
    printf("b: %d\n", b);
    printf("size of int: %lu\n", sizeof(int));
    return 0;
}