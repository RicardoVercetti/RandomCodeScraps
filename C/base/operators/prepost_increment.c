#include <stdio.h>

// if y = ++x  → evaluate ++x before assigning to y
// if y = x++  → assign value to y then evaluate x++       

int main() {

    puts("here goes nothing...");

    int x, y;

    // prefix
    x = 1;
    y = ++x;

    puts("prefix:");
    printf("y= %d\n", y);
    printf("x= %d\n", x);

    // postfix

    x = 1;
    y = x++;

    puts("postfix:");
    printf("y= %d\n", y);
    printf("x= %d\n", x);

    return 0;
}