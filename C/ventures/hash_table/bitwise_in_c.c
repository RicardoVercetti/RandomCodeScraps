#include <stdio.h>
#include <limits.h>

// types: 
// 1. & → bitwise AND
// 2. | → bitwise OR
// 3. ^ → bitwise XOR
// 4. ~ → bitwise NOT
// 5. << → leftshift
// 6. >> → rightshift


void demo() {
    printf("uint max: %u\n", UINT_MAX);
    unsigned int x = 4294967295;
    printf("size: %zu\n", sizeof(x));
    printf("x: %032b\n", x);
}

void basic_all_bitwise_ops() {
    int a = 5;
    int b = 9;

    printf("a: %08b\n", a);
    printf("b: %08b\n", b);

    printf("a&b: %08b\n", a & b);
    printf("a|b: %08b\n", a | b);
    printf("a^b: %08b\n", a^b);


    printf("~a: %08b\n", a = ~a);

    printf("b<<1: %08b\n", b << 1);
    printf("b>>1: %032b\n", b >> 1);
}

int main() {
    puts("hello from the bitwise operation section...!");

    // negative numbers in bin
    int a = -3;
    int b = -2;
    int c = 1;
    unsigned int d = ~ c;


    printf("a: %08b\n", a);
    printf("b: %08b\n", b);
    printf("c: %08b\n", c);
    printf("d: %08b\n", d);
    printf("signed result: %d\n", ~c);
    printf("unsigned result: %u\n", ~c);

    return 0;
}