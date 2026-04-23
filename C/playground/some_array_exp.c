#include <stdio.h>
#include <string.h>

int main() {
    printf("here goes nothing...\n");
    char * value = "some string";
    int len = strlen(value);

    printf("string is: '%s'\n", value);
    printf("length of string: %d\n", len);

    printf("trying to replace some byte of it...\n");
    
    char one = value[5];

    printf("one value: %c\n", one);

    value[5] = 'n';         // this creates segmentation fault, can't replace the string stored in load memory I guess


    printf("after the one char replacement...\n");

    printf("full string: '%s'\n", value);

    return 0;
}