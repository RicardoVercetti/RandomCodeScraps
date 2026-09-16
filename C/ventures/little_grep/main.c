#include <stdio.h>
#include <string.h>
#include <stdbool.h>

// # info
// for cli, the first input is the binary name
// and pieped inputs are to be read from stdin stream of the porgram

// TODO:
// 1. take stdin and search for text mentioned in the cli
// 2. search by each line, and find results by line
// 3. print the result lines highlighted in red
// 4. if -a is passed in the command liner, print all lines with highlight on found line

// skills:
// [  ] colors in console outs
// [  ] cli parsing 

// things to know
// 1. [  ] play with structs
// 2. [  ] booleans are kinda wierd here, why import? then where does the return values for if conditions come from?
// 3. [  ] arrays seems to be differently used than what I'm familiar with

#define MAX_SIZE_STR 50

struct MyGrepParams {
    bool is_all_string;
    char search_string[MAX_SIZE_STR];       // maybe this should be heap allocated string 
};

struct MyGrepParams parseParams(int argc, char *argv[]) {
    struct MyGrepParams params;
    char* all_flag = "-a";
    bool is_value_already_set = false;


    for (int i=1; i<argc; i++) {
        if (strcmp(all_flag, argv[i]) == 0) {
            params.is_all_string = true;
        } else {
            // this must be the string for search
            // there cannot be more than one of these
            if (!is_value_already_set) {
                strcpy(params.search_string, argv[i]);
                is_value_already_set = true;
            }
        }
    }
    return params;
}

int main(int argc, char *argv[]) {
    printf("application runs though...\n");
    // printf("size: %d\n", argc);
    // printf("value1: %s\n", argv[0]);
    // printf("value2: %s\n", argv[1]);


    // char inp[1024];

    // int ret = *fgets(inp, sizeof(inp), stdin);

    // printf("ret int: %x\n", ret);
    // printf("str from stdid: %s\n", inp);

    // parse commmand line for text and flags
    // read things stdin stream and process by line
    // look for string matches and print to console in color for matched string
    struct MyGrepParams params;
    params = parseParams(argc, argv);

    printf("is_all_string: %d\n", params.is_all_string);
    printf("search_string: %s\n", params.search_string);

    return 0;
}


