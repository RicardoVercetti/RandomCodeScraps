#include <stdio.h>
#include "library.h"

// PTR:
// 1. the syntax `<lib.h>` will make the preprocessor look in default directory, 
//    to make it look from current, should use double quotes → `"#include lib.c"`
// 2. if the file included is not a header file, the compiler will expand the contents
//    and so, having two compilations of same code will cause problem.
// 3. static in a function is for internal use inside the same file. Any other .c file
//    having the same name will not create colletion when compiling

int main() {
    printf("program started...\n");
    printf("trying to call the lib function..\n");
    int returned_val = runner();
    printf("responese from the lib: %d\n", returned_val);
    printf("everything finised...\n");
}