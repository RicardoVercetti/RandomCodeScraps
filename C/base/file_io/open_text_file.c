#include <stdio.h>

int main() {
    puts("open text file...");

    char* filename = "notes.md";
    FILE *fp = fopen(filename, "r");

    if (fp == NULL) {
        printf("Unable to open file!\n");
        return 1;
    }
    printf("opened file\n");


    // read character by character

    char ch;
    int count = 0;
    while ((ch = fgetc(fp)) != EOF) {               
        printf("%c", ch);
        // putchar(ch);            // putchar automatically puts it into stdout
        // printf("%s", ch);                            // %s on a char is a bad idea, must use *char[] for %s
        count++;
    }
    printf("\ncount: %d", count);

    // printf("readdata: %s\n", ch);


    int cls = fclose(fp);
    if (cls == 0) {
        printf("\nfile closed successfully...\n");
        return 0;
    }
    printf("Error when closing the file: %d\n", cls);
    return 1;
}