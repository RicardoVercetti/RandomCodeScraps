sys_exit equ 1
sys_write equ 4
stdin equ 0
stdout equ 1

section .data
    msg1 db 'Hello, Programmers', 10
    len1 equ $ - msg1
    
    msg2 db 'Welcome to the world of', 10
    len2 equ $ - msg2

    msg3 db 'Linux assembly programming', 10
    len3 equ $ - msg3

section .text
    global _start

_start:
    ; msg 1
    mov eax, sys_write
    mov ebx, stdout
    mov ecx, msg1
    mov edx, len1
    int 80h

    mov eax, sys_write
    mov ebx, stdout
    mov ecx, msg2
    mov edx, len2
    int 80h

    ; msg 3
    mov eax, sys_write
    mov ebx, stdout
    mov ecx, msg3
    mov edx, len3
    int 80h

    ; exit
    mov eax, sys_exit
    mov ebx, 0
    int 80h
