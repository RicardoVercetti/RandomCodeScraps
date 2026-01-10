#include <stdio.h>
#include <stdbool.h>

int main() {
    printf("yooo....\n");

    // bool is_running = true;
    // bool is_shared = false;

    // printf("is_running: %d\n", is_running);
    // printf("is_shared: %d\n", is_shared);
    // printf("size of bool: %d bytes\n", sizeof(bool));

    bool success = false;
    puts(success ? "true" : "false");

    success = true;
    puts(success ? "true": "false");

    int ret = puts("here is some more puts...");
    printf("return value: %d\n", ret);

    return 0;
}