#include <stdio.h>

// for cli, the first input is the binary name
// and pieped inputs are to be read from stdin stream of the porgram

int main(int argc, char *argv[]) {
    printf("application runs though...\n");
    printf("size: %d\n", argc);
    printf("value1: %s\n", argv[0]);
    printf("value2: %s\n", argv[1]);


    char inp[1024];

    int ret = *fgets(inp, sizeof(inp), stdin);

    printf("ret int: %x\n", ret);
    printf("str from stdid: %s\n", inp);
}