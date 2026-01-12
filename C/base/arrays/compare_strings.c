#include <stdio.h>
#include <string.h>

int main() {

    puts("compare two strings in C...");

    char sender[25] = "Alice",
         receiver[25] = "Bob";

    int result = strcmp(sender, receiver);

    printf("%d\n", result);

    return 0;
}