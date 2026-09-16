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
// colors in terminal
// cli parsing 

// things to know
// 1. [  ] play with structs
// 2. [  ] booleans are kinda wierd here, why import? then where does the return values for if conditions come from?
// 3. [  ] arrays seems to be differently used than what I'm familiar with


// struct MyGrepParams {
//     bool is_all_string;
//     char[] search_string;
// };

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

    return 0;
}

// MyGrepParams parseParams(int argc, char *argv[]) {
//     struct MyGrepParams params;
//     char* all_flag = "-a";
//     bool is_value_already_set = false;


//     for (int i=1; i<argc; i++) {
//         if (strcomp(all_flag, argv[i])) {
//             params.is_all_string = true;
//         } else {
//             // this must be the string for search
//             // there cannot be more than one of these
//             if (!is_value_already_set) {
//                 params.search_string = argv[i];
//             }
//         }
//     }
//     return params;
// }

