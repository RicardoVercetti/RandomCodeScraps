#include <stdio.h>

int square_ref(int *a) {
    printf("a inside fn: %p\n", (void *)&a);
    printf("a deref: %p\n", (void *)&*a);
    return *a * *a;
}

void by_ref() {
    int a = 10;
    printf("pass by reference play!\n");

    printf("value a: %d\n", a);

    int *a_ref = &a;
    printf("a ref from out: %p\n", (void *)&a_ref);

    int s = square_ref(a_ref);
    printf("s: %d\n", s);
    printf("location of s: %p\n", (void *)&s);
    printf("location of a: %p\n", (void *)&a);
}

int square_val(int a) {
    printf("addr: %p\n", &a);
    int res = a * a;
    printf("res addr inside: %p\n", &res);
    return a * a;
}

int main() {
    puts("pass by value in C...");
    
    printf("pass by value tryout!\n");

    int a = 20;

    int s = square_val(a);

    printf("res addr outside: %p\n", &s);


    return 0;
}