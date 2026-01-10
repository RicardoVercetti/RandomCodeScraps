#include <stdio.h>
#include <float.h>

int main() {
    printf("storage size: %d bytes\n", sizeof(float));
    printf("Minimum float value: %E\n", FLT_MIN);
    printf("Maximum float value: %E\n", FLT_MAX);
    printf("Precision: %d decimal digits\n", FLT_DIG);

    puts("\nExample of float precision:\n");

    double d = 12345.6;
    float  f = (float)d;        // how does casting even work in C?

    printf("The floating-point number d 18.10f\n", d);

    printf("Stored in variable f of type float as the values %18.10f\n", f);

    printf("Size of double: %d bytes\n", sizeof(double));
    return 0;
}