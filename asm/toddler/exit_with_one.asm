section .data
    txt db "application started", 10
    len equ $ - txt                     ; length of string
    len_str db "length: ", 10
    len_str_len equ $ - len_str

section .text
    global _start

_start:
    mov eax, 4              ; sys_write
    mov ebx, 1              ; 1 → stdout
    mov ecx, txt            ; text
    mov edx, len            ; length of the text
    int 0x80                ; call kernel

    mov eax, 4
    mov ebx, 1
    mov ecx, len_str        ; length cap str
    mov edx, len_str_len
    int 0x80

    mov eax, 4
    mov ebx, 1
    mov ecx, len             ; length value
    int 0x80

    mov eax, 1              ; sys_exit
    mov ebx, 0              ; exit code
    int 0x80                ; call kernel
