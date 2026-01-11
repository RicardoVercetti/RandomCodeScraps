#include <stdio.h>

#define MAX_NUM 10

int main() {
    puts("Goto statement in C...");

    int needle;

    printf("Please enter a number (0-10):");
    scanf("%d", &needle);

    int i;
    for(i = 0; i < MAX_NUM; i++) {
        if (i == needle) {
            goto end;
        } else {
            printf("Current number: %d\n", i);
        }
    }

    puts("Loop terminated normally.");

    end: printf("Jumped from the goto statement\n");

    return 0;
}

// ideas with the goto statement