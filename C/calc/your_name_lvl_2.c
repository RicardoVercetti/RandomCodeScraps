#include <stdio.h>
#include <string.h>  // for strlen

int main() {
    char name[50];
    printf("Enter your name: ");
    fgets(name, sizeof(name), stdin);  // Read input from stdin (keyboard)

    // Remove the newline character if present
    size_t length = strlen(name);
    if (length > 0 && name[length - 1] == '\n') {
        name[length - 1] = '\0';  // Replace newline with null terminator
    }

    printf("Hello, %s!\n", name);
    return 0;
}
