#include <stdio.h>

void count_down(int n) {
    printf("count: %d\n", n--);
    if (n <= 0) {
        return;
    } 
    count_down(n);
}

void first_sample() {
    
    count_down(3);
    // return 0;
}

int sum_of(int n) {
    printf("n: %d\n", n);
    if (n <= 0) {
        return 0;
    }
    return n + sum_of(n-1);
}

int main() {
    puts("Recursive function to calculate sum of first n natural numbers...");

    int nat = 100000;
    int res = sum_of(nat);
    printf("sum of first %d natural numbers is: %d\n", nat, res);
    
    return 0;
}