section .data
    star db 'the start...', 10
    star_len equ $ - star
    is_even db 'the value is even', 10
    is_even_len equ $ - is_even
    is_odd db 'the value is odd', 10
    is_odd_len equ $ - is_odd
    end db 'the end', 10
    end_len equ $ - end
section .text
    global _start

_start:
    ; start
    mov eax, 4
    mov ebx, 1
    mov ecx, star
    mov edx, star_len
    int 80h

    ; the first operation
    mov ax, 6
    and ax, 1

    ; if zero, jump
    jz evnn

    ; else, its odd
    mov eax, 4
    mov ebx, 1
    mov ecx, is_odd
    mov edx, is_odd_len
    int 80h

    ; sys_exit
    mov eax, 1
    mov ebx, 0
    int 80h             ; not including the kernel call at the end throws the `segmentation fault(core dumped)` error

evnn:
    mov eax, 4
    mov ebx, 1
    mov ecx, is_even
    mov edx, is_even_len
    int 80h
