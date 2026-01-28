#include <stdio.h>
#include <string.h>

typedef struct {
    char* key;
    int value;
} item;

void display_item_array(item *items, size_t size) { // its always a single pointer to the first item in the list/array
    for (int i=0; i<size; i++) {
        // printf("item: %d\n", i);
        printf("key: '%s', value: %d\n", items[i].key, items[i].value);
    }
}

item* linear_search(item* items, size_t size, char* key) {  // here char* is used instead of char[] cuz any array is passed as a pointer to first element, just that char array is null terminated.
    for(int i=0; i<size, i++) {
        printf("item being compared: %s\n", items[i].key);

        int stat = strcomp(items[i].key, key);
        if(stat == 0) {
            printf("the value is equal: %s\n", items[i].key);
            return &items[i];
        } else {
            printf("the else condition: %d\n", stat);
        }
    }
    return NULL;    // prolly this returns a null pointer
}

int main() {
    printf("linear search implementation...\n");

    item items[] = {
        {
            "foo",
            1
        },
        {
            "bar",
            2
        },
        {
            "bazz",
            3
        },
        {
            "buzz",
            4
        },
        {
            "bob", 
            5
        },
        {
            "jane",
            100
        },
        {
            "x",
            200
        }
    };

    size_t size = sizeof(items) / sizeof(item);

    printf("size: %d\n", size);

    // display_item_array(items, size);
    item* found_item = linear_search(items, size, "jane");

    if (found_item == NULL) {
        printf("item not found!\n");
    } else {
        printf("item found!\n");
        printf("key: %s, value: %d\n", *found_item.key, *found_item.value);
    }

    return 0;
}

