# C from scratch

### Why?
> Hopefully this helps to understand more about why certain things are done the way they are in rust.

### Basics

- `stdio.h` is the standard input/output library in C
- `#define VAR_NAME 3.14` is the way of defining static variables
- can also do `const double SALES_TAX = 0.1` for the same behavior
- `%d` for digits(int)
- `%f` for floating point numbers
- `%p` for pointers
- `%s` for strings
- `%c` for char
- float takes about 4 bytes of storage(can be checked with `sizeof(float)`)
- the `%` modulo operator does not work on float or double, works only with int, long, short, long long
- strlen, strcpy, strcomp → from `#include <string.h>`
- `fgets` takes `(char[], sizeof(char[]), stdin)` and adds `\n`, for a string, this must be `\0`.
- when no params are passed in `printf("%d, %d");`, it automatically gets items from stack
- doing a `fn("%p", &var)` might actually work on my machine, but the right way is `fn("%p", (void *)&var)`
- everything that's passed to a function is stack allocated and auto destroyed when the function execution is over. Even a pointer becomes stack allocated.
- auto allocation is stack, manual allocation using `malloc()` requires manually freeing the memory with `free()`
- using `%s` for char is a bad idea, sometime may work, mostly doesn't. `%s` format specifier requires a `*char[]` and prints the string repr one by one until `\0` is found. 

### Building the code

- `gcc -c file.c` compiles the c code into object files
- This object file then can be linked together with `gcc file.o -o main`. This will give the the binary `./main`
- must include the `.h` file int the `.c` file. And wherever the .c file's code have to be reused, include the `.h` file.
- function declarations at the `.h` file are just a promise. When linker does its job, it does not check if the promise is kept 
- `gcc file1.c file2.c -o main` to do both compiling and linking together.

### Bitwise operators

| C Bitwise operators   | Description           |
|-----------------------|-----------------------|
| &                     | Bitwise AND           |
| \|                    | Bitwise OR            |
| ^                     | Bitwise ex-OR         |
| <<                    | Bitwise left shift    |
| >>                    | Bitwise right shift   |
| ~                     | One's complement      |


### Datatypes unlocked

- int
- double → double precision floating point(8bytes)
- float → single precision floating point(4bytes)
- char → just integer behind the scenes(1byte)
- bool → any integer other than 0 is true(can use `stdbool.h` for constant `0` or `1`)
- string → array of char is a string, end of string is null terminator `\0`.

### Datatype overview

| Basic Types | Derived Types | Enum    | Void      |
|-------------|---------------|---------|-----------|
| char        | array         |         |           |
| int         | pointer       |         |           |
| float       | struct        |         |           |
| double      | union         |         |           |




#### Reserved keywords

| col A     | col B     | col C     | col D     |
|-----------|-----------|-----------|-----------|
| auto      | break     | int       | return    |
| case      | char      | register  | signed    |
| const     | continue  | short     | static    |
| default   | do        | sizeof    | switch    | 
| double    | else      | struct    | union     |
| enum      | extern    | typedef   | void      |
| float     | for       | unsigned  | while     |
| goto      | if        | volatile  |           |

### Outstanding 

- [ ] left-shift operation
- [ ] right-shift operation
- [ ] bitwise AND operation
- [ ] bitwise exclusive OR operation
- [ ] bitwise inclusive OR operation
- [ ] unsigned int vs signed int
- [ ] DSA