#include <stdio.h>

int main() {
    puts("file I/O in C...");

    FILE *f = fopen("notes.md", "r");

    if (f == NULL) {
        printf("Couldn't open file!\n");
        return 1;
    }

    printf("addr: %p\n", f);

    int res = fclose(f);
    printf("fclose int: %d\n", res);

    return 0;
}