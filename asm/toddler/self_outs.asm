section .data
    msg db 'Hello all', 0xa         ; string to be printed
    len equ $ - msg                 ; length of string

section .text
    global _start

_start:
    mov edx, len        ; message length
    mov ecx, msg        ; message to write
    mov ebx, 1          ; file descriptor (stdout)
    mov eax, 4          ; sys_write
    int 0x80            ; call kernel

    mov eax, 1          ; sys_exit
    int 0x80            ; call kernel
