#include <stdio.h>
#include <string.h>

// Notes:
// 1. uninitialized variables can have arbitary zero(in bits) value. Therefore always initialized value else it's considered undefined.
// 2. returning an owned string seems to have different memory address for values comparing inside & outside the functions.
// 3. seems the string passed into function seems different - not owned array of chars but one char pointer is the way.


struct Stack {
    int val;
    bool is_okay;
};


struct Stack from_function() {
    struct Stack s1;
    s1.val = 22;
    s1.is_okay = true;

    printf("values inside: %x\n", s1.val);
    printf("value in bool: %x\n", s1.is_okay);
    printf("address inside: %x\n", &s1);
    printf("address for first item: %x\n", &s1.val);
    printf("addres for the second item: %x\n", &s1.is_okay);
    return s1;
}

void string_into_function(char* start) {
    printf("string passed: %s\n", start);
}


int main() {
    printf("runzz...\n");

    struct MyStruct {
        int myNum;
        char myChar;
    };

    struct MyStruct s1;

    s1.myNum = 4;
    // s1.myChar = '5';


    printf("myNum: %.8b\n", s1.myNum);
    printf("myChar: %b\n", s1.myChar);

    printf("sizeof: %d\n", sizeof(s1));
    printf("sizeof int: %d\n", sizeof(int));
    printf("sizeof char: %d\n", sizeof(char));


    struct Stack s2 = from_function();

    printf("fromm func: %x\n", s2.val);
    printf("fromm func bool: %x\n", s2.is_okay);
    printf("addres outside: %x\n", &s2);
    printf("address outside for first item: %x\n", &s2.val);
    printf("address outside for second item: %x\n", &s2.is_okay);


    // strings
    char some_string[] = "here is some string...";
    printf("str value: '%s'\n", some_string);
    printf("length: %d\n", strlen(some_string));

    string_into_function(some_string);

    return 0;
}


