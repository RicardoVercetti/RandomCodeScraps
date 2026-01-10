#include <stdio.h>

int main() {
    // + - * / %

    printf("Arithmetic in C...\n");

    float x = 5, y = 3;

    printf("%f + %f = %f\n", x, y, x+y);
    printf("%f - %f = %f\n", x, y, x-y);
    printf("%f * %f = %f\n", x, y, x*y);
    printf("%f / %f = %f\n", x, y, x/y);
    // printf("%f %% %f = %f\n", x, y, x%y);      // mod doesn't work on anything other than straight up integers like int, char, short, long, long long

    return 0;
}