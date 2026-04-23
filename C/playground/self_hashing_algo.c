#include <stdio.h>
#include <string.h>

void display_hex(char * arr, int size) {
    printf("in hex: ");
    for (int i=0; i<size; i++) {
        printf("%x", arr[i]);
    }
    printf("\n");
}

int main() {
    printf("my own hashing algorithm...\n");
    // approach:
    // 1. have a fixed length of byte array, same length as the output
    // 2. for each char, try doing a and operation with all the items in the fixed byte array
    // (missing the avalanche effect)
    // 3. the resultant fixed length byte array would be the hash

    char * arr = "abcdefghijklmnop";
    int len = strlen(arr);
    printf("len by the strlen() = %d\n", len);

    char * input = "here is a text to hash";
    int inp_len = strlen(input);
    printf("input length: %d\n", inp_len);

    printf("looping through the string...\n");

    printf("-- before --\n");
    display_hex(arr, len);

    for(int n=0; n<inp_len; n++) {
        // printf("value at (%d): %c\n", n, arr[n]);
        // printf("bin: %08b, hex: %x, int: %d\n", arr[n], arr[n], arr[n]);
        // here do the or and operation for each
        printf("outer loop: %d\n", n);
        for(int i=0; i<len; i++) {
            char and = arr[i] & input[n];
            printf("left: %08b, right: %08b\n", arr[i], input[n]);
            printf("oped: %08b\n", and);
            printf("inner loop: %d \n", i);
            *(arr+i) = and;
        }
    }

    printf("-- after --\n");
    display_hex(arr, len);
    

    return 0;
}
