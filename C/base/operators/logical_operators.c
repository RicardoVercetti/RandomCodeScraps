#include <stdio.h>

int main() {

    puts("logical operators time...");

    int a = 1;
    int b = 4;
    char c = 'C';

    int res = a < b;
    int char_res = c > b;


    printf("res: %c\n", res);
    printf("char: %c\n", c);
    printf("char res: %c\n", char_res);

    return 0;
}