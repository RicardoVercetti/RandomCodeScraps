#include <stdio.h>

/// the includes that I've seen from other codes
// <ctype.h>
// <stdio.h>
// <stdlib.h>
// <string.h>
// <time.h>

struct item {
    char* key;
    int value;
};


int main() {
    printf("trying out linear search...\n");

    // struct item it = { "key", 10 };

    // printf("value : %d, key: '%s'\n", it.value, it.key);

    printf("size: %zu\n", sizeof(char));
    char c = '0';

    printf("value in bin: %b\n", 200);
    printf("value in hex: %x\n", 200);

    printf("single char: %d\n", c);
    printf("single char in hex: %x\n", c);
    printf("single char in ptr: %p\n", c);

    printf("value of NULL: %d\n", NULL);

    int* n = ((void *) 0);

    printf("address of n: %p\n", &n);

    if (n == NULL) {
        printf("the N is NULL yoo...\n");
    } else {
        printf("the N is not NULL: %d\n", n);
    }

    // printf("n value: %d\n", *n);
    // printf("n in hex: %x\n", *n);

    return 0;
}