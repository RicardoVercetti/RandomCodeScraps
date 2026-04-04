section .data
    userMsg db 'Please enter a number: '
    lenUsrMsg equ $ - userMsg
    dispMsg db 'You have entered: '
    lenDispMsg equ $ - dispMsg

section .bss        ; Uninitialized data
    num resb 5

section .text
    global _start

_start:
    ; user prompt
    mov eax, 4
    mov ebx, 1
    mov ecx, userMsg
    mov edx, lenUsrMsg
    int 0x80

    ; take input from the user and store the same
    mov eax, 3
    mov ebx, 2
    mov ecx, num
    mov edx, 5          ; 5 bytes(1 for sign) of that information
    int 0x80

    ; output the display message
    mov eax, 4
    mov ebx, 1
    mov ecx, dispMsg
    mov edx, lenDispMsg
    int 0x80

    ; output the input value that has been captured
    mov eax, 4
    mov ebx, 1
    mov ecx, num
    mov edx, 5          ; let's try printing just one value
    int 0x80

    ; new line character before sys_exit
    mov eax, 4
    mov ebx, 1
    mov ecx, 10
    int 0x80

    ; sys_exit
    mov eax, 1
    mov ebx, 0
    int 0x80
