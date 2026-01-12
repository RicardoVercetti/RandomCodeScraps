#include <stdio.h>
#include <string.h>

int main() {

    puts("reading string from keyboard...");
    
    char message[100];

    printf("sizeof message: %d\n", sizeof(message));
    printf("strlen: %d\n", strlen(message));

    fgets(message, sizeof(message), stdin);     // by default fgets gives a '\n' at the end of the input
    message[strlen(message)-1] = '\0';          // therefore replacing it with the null exterminator is the way to go

    printf("message: '%s'\n", message);
    printf("length: %d\n", strlen(message));

    // lets try displaying by each character
    for (int i = 0; i < strlen(message); i++) {
        if (message[i] == '\n') {
            printf("message[%d] = NEW_LINE\n", i);
        } else if (message[i] == '\0') {
            printf("message[%d] = NULL\n", i);
        } else {
            printf("message[%d] = %c\n", i, message[i]);
        }
    }

    int last_item_pos = strlen(message);
    printf("last pos: %d\n", last_item_pos);
    printf("one after that: %d\n", message[last_item_pos]);
    char n_after_that[10];
    fgets(n_after_that, sizeof(n_after_that), stdin);

    printf("n_after_that: %d\n", n_after_that);

    return 0;
}