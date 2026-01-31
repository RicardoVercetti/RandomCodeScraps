#include <stdio.h>
#include <stdlib.h>

// <stdlib.h> has malloc(), calloc() and realloc()

int main() {
    printf("heap allocation tryout in C...\n");

    int *p = malloc(sizeof(int) * 10);
    if (p == NULL) {
        printf("memory allocation failed...\n");
        exit(1);
    }

    printf("allocated address: %x\n", *p);

    return 0;
}

// Questions:
// 1. what if I wanna allocate my custom type of arbitrary length? should I have the struct type at the front when I have the pointer like `MyStruct *mp`
// 2. what can I do with something like `int *p = malloc(sizeof(int) * 10);`
//          - given the size of int(4 bytes), this memory would be 40 bytes in continuous memory
//          - what can I do with this? I cant allocate an integer to it, can I?
//          - given the pointer `*p` points to the first position in the memory, can I keep adding char(1 bytes) until its full?
// 3. why is the datatype even necessary when having a pointer? anyway its an 8byte address right? and it does'nt even have data on how large is this continuous array of memory allocated