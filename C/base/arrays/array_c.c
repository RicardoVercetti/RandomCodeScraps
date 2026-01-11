#include <stdio.h>

int main() {
    puts("Arrays in C...");

    int size = 10;
    int numbers[size];

    int i;
    // create array from user input
    for(i=0; i<size; i++) {
        printf("Enter %dth number: ", i+1);
        scanf("%d", &numbers[i]);
    }

    // printf("here are the items you've given: '%d'\n", numbers);
    printf("sizeof: %d\n", sizeof(numbers));

    for(i=0; i<size; i++) {
        printf(i != size-1 ? "%d, " : "%d", numbers[i]);
    }

    printf("\n");
    // find the sum of all the items
    int total;
    for(i=0; i<size; i++) {
        total += numbers[i];
    }

    printf("sum of all numbers: %d\n", total);

    return 0;
}