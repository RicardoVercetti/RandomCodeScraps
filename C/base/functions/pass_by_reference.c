#include <stdio.h>

void swap_values(int *a, int *b) {
    int temp = *a;
    *a = *b;
    *b = temp;
}

int main() {

    printf("Pass by reference swap...\n");

    // create two variables
    // pass it inside a swap function
    // swap the values using a temp variable
    // print and show the values out side the function

    int a = 10, b = 20;

    printf("values before swap: (a, b) = (%d, %d)\n", a, b);

    swap_values(&a, &b);

    printf("values after swap: (a, b) = (%d, %d)\n", a, b);

    return 0;
}