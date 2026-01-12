#include <stdio.h>

int max(int a, int b);

int main() {
    puts("functions example in C...");


    // get two number inputs in two different variables
    int a, b;
    printf("enter the A: ");
    scanf("%d", &a);
    printf("enter the B: ");
    scanf("%d", &b);

    printf("you entered A|B = %d|%d\n", a, b);
    // implement max function that takes two integers and returns the max int

    int max_num = max(a, b);
    // print the result

    printf("max num is: %d\n", max_num);

    return 0;
}

int max(int a, int b) {
    if (a > b) {
        return a;
    }
    return b;
}

