#include <stdio.h>
#include <string.h>

int main() {
    char name[50];
    printf("Enter your name: ");
    fgets(name, sizeof(name), stdin);

    size_t length = strlen(name);
    if (length > 0 && name[length - 1] == '\n') {
        name[length - 1] = '\0';
    }

    printf("Hello,%s!\n", name);
    // fflush(stdout);
    return 0;
}
