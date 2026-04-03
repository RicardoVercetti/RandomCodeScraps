# ASM notes

## tools
- nasm: assembler
- gcc: linker to create executables
- glibc-devel.i686: needed for 32 bit programs
- gdb: debugger

## commands to compile the code

- `nasm -f elf32 filename.asm`
- `gcc -m32 filename.o -o outfilename`

## segments

1. vars

```asm
section .data
    msg db "Hello, World!", 10
    len equ $ - msg
```

2. instructions

```asm
section .text
    global _start
```


## questions:

- what does the `len equ $ - txt` do?
- 