#include <stdio.h>

// #define CONSTANT_NAME value         // this is a C preprocessor statement
#define PI 3.14

int main() {
    int radius = 2;

    const double SALES_TAX = 0.1;

    float area = PI * radius * radius;
    printf("hello from variables...\n");
    printf("area: %0.5f\n", area);
    printf("tax: %0.2f\n", SALES_TAX);
    return 0;
}