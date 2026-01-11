#include <stdio.h>

int main() {

    puts("bitwise operators in C...");

    // bitwise operators can only be applied to char, short, int & long


    int d1 = 4,     // 100 
        d2 = 6,     // 110
        d3;

    printf("d1 = %d, d2 = %d\n", d1, d2);

    d3 = d2 & d1;
    printf("Bitwise AND d1 & d2 = %d\n", d3);

    d3 = d1 | d2;
    printf("Bitwise OR d1 | d2 = %d\n", d3);

    d3 = d1 ^ d2;
    printf("Bitwise XOR d1 ^ d2 = %d\n", d3);

    d3 = ~d1;
    printf("One's complement of d1 = %d\n", d3);

    d3 = d1 << 2;
    printf("Left shift by 2 bits d1 << 2 = %d\n", d3);

    d3 = d1 >> 2;
    printf("Right shift by 2 bits d1 >> 2 = %d\n", d3);

    // printf("binary :%b\n", 23); // 10_111  2(4) + 2(0) + 2(2) + 2(1) + 2(0)
                                //          16  + 1    + 4    + 2    + 1
    return 0;
}