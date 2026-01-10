#include <stdio.h>

int main() {

    puts("lets try enums in C...");

    enum STATUS {
        open = 4,
        assigned,
        fixed
    } bug_status;

    enum STATUS stat = assigned;

    printf("status: %d\n", stat);

    return 0;
}