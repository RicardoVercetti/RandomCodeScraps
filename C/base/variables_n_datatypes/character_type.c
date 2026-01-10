#include <stdio.h>

int main() {

    char key = 'A';
    char val = 66;
    char zero = 32;

    printf("hi from the character intro...\n");
    printf("size of char: %d bytes\n", sizeof(char));

    printf("char character: %c\n", key);
    printf("val character: %c\n", val);
    printf("zero character: '%c'\n", zero);
    return 0;
}