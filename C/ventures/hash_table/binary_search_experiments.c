#include <stdio.h>
#include <unistd.h>

// items
typedef struct {
    char* key;
    int value;
} item;

void display_item(item* items, size_t size) {
    printf("counts: ");
    for(int i=0; i<size; i++) {
        printf("%s", items[i].key);
        if (!(size-1 == i)) {
            printf(", ");
        } else {
            printf(".");
        }
    }
    printf("\n");
}

item* binary_search(item* items, size_t size, item* findable) {
    // go from middle, if pos of finable is equal, return this, if less than middle, go left, else go right
    int to_find = findable->value;
    int mid = (size+2-1)/2;
    int start_pos = 0;
    int end_pos = size;
    printf("findable: %d\n", to_find);

    while(start_pos < end_pos && (start_pos + 1 != end_pos)) {
        printf("start_pos: %d, end_pos: %d, mid: %d\n", start_pos, end_pos, mid);

        // printf("mid pos: %d\n", mid);
        // get mid and check
        if (items[mid].value == to_find) {
            return &items[mid];
        } else if (items[mid].value < to_find ) {    // mid is small, look right
            printf("mid is small, looking right\n");
            start_pos = mid;
            mid = start_pos + (end_pos - start_pos) / 2;
            // continue;
        } else {    // mid is large, look left
            printf("mid is large, looking left\n");
            end_pos = mid;
            mid = start_pos + (end_pos - start_pos) / 2;
            // continue;
        }
        // just keep changing the start_pos & end_pos until the to_find is found
        
        sleep(1);
    }

    return NULL;
}

int main() {
    printf("binary search experiments...\n");

    item items[] = {
        { "one", 1 }, { "two", 2}, { "three", 3 }, { "four", 4 }, { "five", 5 }, { "six", 6 }, { "seven", 7 }, 
        { "eight", 8 }, { "nine", 9 }, { "ten", 10 }
    };

    size_t size = sizeof(items)/sizeof(item);
    printf("array len: %zu\n", size);

    // display_item(items, size);
    item* found_item = binary_search(items, size, &items[7]);

    if (found_item!= NULL) {
        printf("found item: %s, value: %d\n", found_item->key, found_item->value);
    } else {
        printf("item not found...\n");
    }


    return 0;
}

