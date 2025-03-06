#include <stdio.h>

int main() {
    char name[50];
    printf("Enter your namee: ");
    fgets(name, sizeof(name), stdin);  // Read input from stdin (keyboard)
    printf("Hello,%s!", name);
    fflush(stdout);
    return 0;
}
