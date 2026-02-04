#include <stdio.h>
#include <stdint.h>


// code sample:
// static uint64_t hash_key(const char* key) {
//     uint64_t hash = FNV_OFFSET;
//     for (const char* p = key; *p; p++) {
//         hash ^= (uint64_t)(unsigned char)(*p);
//         hash *= FNV_PRIME;
//     }
//     return hash;
// }

// unknowns:
// 1. whats a uint64 data type
//      → this is a fixed size datatype from `stdint.h`, unlike long(that takes 4bytes in some machines, 8 bytes in some other) this always takes 8 bytes
// 2. why is there *p in the for loop syntax?
//      → `\0` evaluate to false in a char array look up. When you have a key data in a char array, you know the last value is always going to be `\0`

void q1_ans() {
    uint64_t hash = 4364UL;
    printf("sizeof uint64_6: %lu\n", sizeof(uint64_t));

    printf("binary: %lb\n", hash);

}

int main() {
    puts("hash algorithm implementation...");
    
    char items[] = "this is some char array";

    printf("size of array: %zu\n", sizeof(items));
    int size = (int)sizeof(items);
    char* item_ptr = items;
    
    // printf("size in int: %d\n", size);
    // for(int i=0; i<size; i++) {
    //     printf("item hex: %x, item char: %c\n", item_ptr[i], item_ptr[i]);
    // }

    // now looping without the length in mind
    for(char* first_item = item_ptr; *first_item; first_item++){
        // printf("(i, item)=(%d, %c)\n", i, *(item_ptr + i));
        printf("char item: %c, hex: %x\n", *first_item, *first_item);
    }

    return 0;
}