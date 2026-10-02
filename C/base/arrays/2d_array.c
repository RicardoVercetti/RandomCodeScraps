#include <stdio.h>

int main() {
    printf("2D arrays in C...\n");
    int arr[3][4] = {
        {1, 2 , 3, 4},
        {5, 6, 7, 8},
        {9, 10, 11, 12},
    };

    printf("digit: %d\n", arr[0][2]);
    printf("sizeof: %zu\n", sizeof(arr));
    printf("sizeof INT: %d\n", sizeof(int));
    printf("last item: %d\n", arr[2][3]);

    return 0;
}