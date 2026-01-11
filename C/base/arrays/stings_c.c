#include <stdio.h>

int main() {

    puts("Strings in C");       // always null('\0') terminated

    char message[6];

    message[0] = 'H';
    message[1] = 'e';
    message[2] = 'l';
    message[3] = 'l';
    message[4] = 'o';
    message[5] = '\0';

    printf("dig: '%d'\n", message[4]);
    printf("str: '%s'\n", message);

    return 0;
}