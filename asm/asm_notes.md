# ASM notes

## tools
- nasm: assembler
- gcc: linker to create executables
- glibc-devel.i686: needed for 32 bit programs
- gdb: debugger

## commands to compile the code

- `nasm -f elf32 filename.asm`
- `gcc -m32 filename.o -o outfilename`

## segments of .asm file

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

## Processor segment

- General registers are: Data registers, Pointer registers, Index registers
- complete 32 bit registers: EAX, EBX, ECX, EDX
- the lower halves can be used as 16 bit registers: AX, BX, CX, DX
- Lower and higher halves of the above-mentioned four 16-bit registers can be used as 8 bit registers: AH, AL, BH, BL, CH, CL, DH and DL

```text
AX - Accumulator
BX - Base
CX - Counter
DX - Data
``` 

- Pointer registers EIP, ESP, EBP: instruction pointer, stack pointer, base pointer
- Control registers:
    - Overflow flag(OF)
    - Direction flag(DF)
    - Interrupt flag(IF)
    - Trap flag(TF)
    - Sign flag(SF)
    - zero flag(ZF)
    - Auxiliary carry flag(AF)
    - Parity flag(PF)
    - Carry flag(CF)

## Linux system calls(are listed in `/usr/include/asm/unistd.h`)

| %eax | Name       | %ebx              | %ecx          | %edx      |
|------|------------|-------------------|---------------|-----------|
| 1    | sys_exit   | int               | -             | -         |
| 2    | sys_fork   | struct pt_regs    | -             | -         |
| 3    | sys_read   | unsigned int      | char *        | size_t    |
| 4    | sys_write  | unsigned int      | const char *  | size_t    |
| 5    | sys_open   | const char *      | int           | int       |
| 6    | sys_close  | unsigned int      | -             | -         |

## Allocating space for initialized data

| Directive |       Purpose         |       storage space       |
|-----------|-----------------------|---------------------------|
|   DB      |   Define byte         |       1 bytes             |
|   DW      |   Define word         |       2 bytes             |
|   DD      |   Define double word  |       4 bytes             |
|   DQ      |   Define Quad word    |       8 bytes             |
|   DT      |   Define ten bytes    |       10 bytes            |

## Allocating for uninitialized data

| Directive   | Purpose                     |
|-------------|-----------------------------|
| RESB        | Reserve a byte              |
| RESW        | Reserve a Word              |
| RESD        | Reserve a Double word       |
| RESQ        | Reserve a Quad word         |
| REST        | Reserve a Ten Bytes         |

## questions:

- what does the `len equ $ - txt` do?
- why is there always a `10` or `0A` after any string literal at the `section .data`
    - its new line character
- how to do new line character if the value we're trying to print is doesn't contain the new line character, basically a string concatenation

## PTRs

- when eax is given the sys_call, ebx is given the the first parameter, the ecx takes a memory address in hex, not literal values.
- and the edx takes the number of bytes after the item that ecx points to 