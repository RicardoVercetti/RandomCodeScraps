# C from scratch

### Why?
> Hopefully this helps to understand more about why certain things are done the way they are in rust.

### Basics

- `stdio.h` is the standard input/output library in C
- `#define VAR_NAME 3.14` is the way of defining static variables
- can also do `const double SALES_TAX = 0.1` for the same behavior
- `%d` for digits(int)
- `%f` for floating point numbers
- float takes about 4 bytes of storage(can be checked with `sizeof(float)`)
- the `%` modulo operator does not work on float or double, works only with 

### Datatypes unlocked

- int
- double → double precision floating point(8bytes)
- float → single precision floating point(4bytes)
- char → just integer behind the scenes(1byte)
- bool → any integer other than 0 is true(can use `stdbool.h` for constant `0` or `1`)

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

