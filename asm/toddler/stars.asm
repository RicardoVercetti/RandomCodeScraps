section .data
    msg db 'Displaying 9 starts but no new line?', 10
    len equ $ - msg             ; length of message
    s2 times 9 db '*'

section .text
    global _start

_start:                 ; tell the linker entry point
    mov eax, 4          ; sys_write
    mov ebx, 1          ; stdout
    mov ecx, msg
    mov edx, len
    int 0x80

    mov eax, 4
    mov ebx, 1
    mov ecx, s2         ; starts
    mov edx, 9          ; length of string
    int 0x80

    mov eax, 4
    mov ebx, 1
    mov ecx, 0          ; to see if we get a new line
    int 0x80

    mov eax, 1          ; sys_exit
    mov ebx, 0
    int 0x80            ; kernel call
