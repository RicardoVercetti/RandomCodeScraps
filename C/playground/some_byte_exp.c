#include <stdio.h>

int main() {
    printf("some byte experiments in C...\n");

    // bytes as integers
    // 1 bytes      → 8 bits
    // 1 int        → 4 bytes
    // 1 char       → 1 byte
    // 1 bool       → ?

    int a = 1;
    char c = 'a';

    int si = sizeof(a);
    int char_size = sizeof(c);

    printf("size of int: %d\n", si);
    printf("size of char: %d\n", char_size);


    return 0;
}