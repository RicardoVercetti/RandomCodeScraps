#include <stdio.h>
#include <string.h>

// " " → is a string literal, but cant directly assign

void sample_1() {
    char message[10];

    strcpy(message, "Hello");

    printf("message: '%s'\n", message);

    int len = strlen(message);
    printf("len: %d\n", len);
}

int main() {

    puts("string operations in C...");

    char first_name[5] = "John",
         last_name[5] = "Doe",
         full_name[6];

    strcpy(full_name, first_name);
    strcat(full_name, " ");
    strcat(full_name, last_name);

    printf("full_name: %s\n", full_name);
    printf("len(should be small): %d\n", strlen(full_name));

    strcat(full_name, "is G");

    printf("full str: %s\n", full_name);
    printf("len now: %d\n", strlen(full_name));

    return 0;
}