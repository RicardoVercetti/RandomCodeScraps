#include <stdio.h>

int main() {
    puts("reading file line by line...");

    char *filename = "notes.md";
    FILE *fp = fopen(filename, "r");

    if (fp == NULL) {
        printf("Could not open file!\n");
        return 1;
    }

    const unsigned MAX_LENGTH = 20;
    char buffer[MAX_LENGTH];

    int count = 0;
    while (fgets(buffer, MAX_LENGTH, fp)) {
        printf("%s", buffer);
        count++;
    }

    int cls = fclose(fp);

    if (cls == 1) {
        printf("Error occurred while trying to close the file!\n");
        return 1;
    }

    printf("\nClosed the file successfully...\n");
    printf("line count: %d\n", count);
    return 0;
}