section .data
    count dw 0
    value db 15
    txt db 'done', 10
    count_text db 'count is: '
    nxt_line db '', 10
    before db 'before: '
    after db 'after: '
    loog db 'long after: '
    len equ $ - loog

section .bss
    num resb 1

section .text
    global _start

_start:

    ; before header
    mov eax, 4
    mov ebx, 1
    mov ecx, before
    mov edx, 8
    int 80h

    ; preprocess data
    mov al, [count]
    add al, '0'
    mov [num], al

    ; value
    mov eax, 4
    mov ebx, 1
    mov ecx, num
    mov edx, 1
    int 80h

    ; new line
    mov eax, 4
    mov ebx, 1
    mov ecx, nxt_line
    mov edx, 1
    int 80h

    inc word [count]
    dec byte [value]

    ; after
    mov eax, 4
    mov ebx, 1
    mov ecx, after
    mov edx, 7
    int 80h

    ; set value
    mov al, [count]
    add al, '0'             ; changes the integer to ascii, this is a must to print the value of integer datatype
    mov [num], al

    ; value
    mov eax, 4
    mov ebx, 1
    mov ecx, num
    mov edx, 1
    int 80h
    
    mov ebx, count
    inc word [ebx]

    mov esi, value
    dec byte [esi]

    ; pretext
    mov eax, 4
    mov ebx, 1
    mov ecx, loog
    mov edx, len
    int 80h

    ; lets see the value
    mov al, [value]
    add al, '0'
    mov [num], al

    mov eax, 4
    mov ebx, 1
    mov ecx, num
    mov edx, 1
    int 80h

    ; lets put some souts just see its running
    mov eax, 4
    mov ebx, 1
    mov ecx, txt
    mov edx, 1
    int 80h

    mov eax, 1
    mov ebx, 0
    int 80h
