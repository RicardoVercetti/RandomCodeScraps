#include <stdio.h>
#include <stdlib.h>

// <stdlib.h> has malloc(), calloc() and realloc() for allocation and  free() for deallocation

int main() {
    printf("heap allocation tryout in C...\n");

    int *p = malloc(sizeof(int) * 10);      // this would be 4 * 10 = 40
    if (p == NULL) {
        printf("memory allocation failed...\n");
        exit(1);
    }

    // char *c = malloc(20);           // 20 bytes, I guess

    *p = 132;
    *(p+1) = 133;
    *(p+2) = 134;
    *(p+3) = 'C';

    printf("allocated address: %p\n", p);
    printf("item in address: %d\n", *p);
    printf("size of P: %zu\n", sizeof(*p));

    printf("second item: %d\n", *(p+1));
    printf("third item: %d\n", *(p+2));
    printf("non existant fourth item: %c\n", *(p+3));

    printf("item before freeing: %d\n", *p);
    printf("second item before freeing: %d\n", *(p+1));
    free(p);

    printf("freed memory!\n");
    printf("first item now: %d\n", *p);
    printf("second item after freeing: %d\n", *(p+1));



    return 0;
}

// Questions:
// 1. what if I wanna allocate my custom type of arbitrary length? should I have the struct type at the front when I have the pointer like `MyStruct *mp` → Yes
// 2. what can I do with something like `int *p = malloc(sizeof(int) * 10);`
//          - given the size of int(4 bytes), this memory would be 40 bytes in continuous memory                    → yes 
//          - what can I do with this? I cant allocate an integer to it, can I?                                     → you can, *p gives the first 4byte location, *(p+1) gives the next four byte location and so on.
//          - given the pointer `*p` points to the first position in the memory, can I keep adding char(1 bytes) until its full?  → size of the byte when doing `*(p+1)` is defined based on the size of the data type, can assign 1 char to `*(p+n)` but cant do multiple chars.
// 3. why is the datatype even necessary when having a pointer? anyway its an 8byte address right? and it does'nt even have data on how large is this continuous array of memory allocated  
//          → Its necessary when walking by the pointer. given a char pointer `*c`, doing `*(c+1)` walks by 1 bytes given the size of char, while an int pointer walks by 4 bytes given the size of int datatype
// 4. The memory allocated is 40 bytes starting from the location of *p, how can I free it? just `free(p)` is enough? for accessing, I had to do `*(p+n)`, dont I have to do this for freeing?
//          → doing `free(p)` is all thats needed it seems like
// 5. Allocating using char/int and giving multiple of its size is basically an array of these values. And when allocated, the programmer have to keep track of the length of this array. Is this how its supposed to be done for an array? 
// 6. What's the difference between `malloc()` and `calloc()`?