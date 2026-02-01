#include <stdio.h>

void some_bitwise() {
    unsigned int a = 1;

    printf("inverse: %u\n", ~a);
    printf("bin a       : %032b\n", a);
    printf("inv bin a   : %b\n", ~a);

    unsigned int b = a ^ ~a;
    printf("b in int: %u\n", b);
    printf("b in bin: %032b\n", b);

    unsigned int ib = ~b;
    printf("inverse B: %u\n", ib);
    printf("bin ib: %032b\n", ib);

    unsigned int sb1 = 1 << 1;
    unsigned int sb2 = 1 << 2;
    unsigned int sb3 = 1 << 3;

    printf("sb0 → (u, b): (%u, %b)\n", 1, 1);           // 1 x 2(0) = 1 x 1                                  = 1
    printf("sb1 → (u, b): (%u, %b)\n", sb1, sb1);       // 2 x 2(1) + 0 x 2(0) = 2 + 0                       = 2 
    printf("sb2 → (u, b): (%u, %b)\n", sb2, sb2);       // 2 x 2(2) + 0 x 2(0) + 0 x 2(0) = 4 + 0 + 0        = 4
    printf("sb3 → (u, b): (%u, %b)\n", sb3, sb3);       // 2x2(3) + 0x2(2) + 0x2(1) + 0x2(0) = 8 + 0 + 0 + 0 = 8

}

int main() {
    puts("more bitwise stuff in C...");

    // L1: negative numbers should not be pushed with << or >>
    int a = -1;
    int pa = a << 4;

    printf("a bin       : %032b\n", a);
    printf("pa bin      : %032b\n", pa);
    printf("pa in int   : %d\n", pa);
    
    return 0;
}

// things to try:
// 1. implementing base 10 arithmetic using the base 2 bitwise operators?