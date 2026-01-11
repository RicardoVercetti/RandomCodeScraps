#include <stdio.h>

int main() {

    printf("char: %d byte(s)\n", sizeof(char));
    printf("short: %d byte(s)\n", sizeof(short));
    printf("int: %d byte(s)\n", sizeof(int));
    printf("long: %d byte(s)\n", sizeof(long));
    printf("long-long: %d byte(s)\n", sizeof(long long));
    printf("bool: %d byte(s)\n", sizeof(bool));

    return 0;
}