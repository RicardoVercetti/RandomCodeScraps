section .data
    CONSTANT_VALUE db '60', 10

section .text
    global _start

_start:
    mov eax, 4
    mov ebx, 1
    mov ecx, CONSTANT_VALUE
    mov edx, 3
    int 80h

    ; exit
    mov eax, 1
    mov ebx, 0
    int 80h
