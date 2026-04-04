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

## questions:

- what does the `len equ $ - txt` do?
- why is there always a `10` or `0A` after any string literal at the `section .data`
    - its new line character