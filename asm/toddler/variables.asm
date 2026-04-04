section .data
    choice db 'y', 10
    text db 'you entered: '
    text_len equ $ - text
    nxt_line db '', 10

section .bss
    inp resb 1

section .text
    global _start

_start:
    mov eax, 4
    mov ebx, 1
    mov ecx, choice
    mov edx, 2
    int 80h

    ; user input
    mov eax, 3
    mov ebx, 2
    mov ecx, inp
    mov edx, 1
    int 80h

    ; out the input pretext
    mov eax, 4
    mov ebx, 1
    mov ecx, text
    mov edx, text_len
    int 80h

    ; out the input value
    mov eax, 4
    mov ebx, 1
    mov ecx, inp
    mov edx, 1
    int 80h

    ; just a new line baby
    mov eax, 4
    mov ebx, 1
    mov ecx, nxt_line
    mov edx, 1
    int 80h

    ; exit
    mov eax, 1
    mov ebx, 0
    int 80h
