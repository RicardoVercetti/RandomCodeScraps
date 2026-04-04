section .data
    name db 'Zara Ali '
    name_len equ $ - name

section .text
    global _start

_start:
    ; writing name zara Ali
    mov eax, 4
    mov ebx, 1
    mov ecx, name
    mov edx, name_len
    int 80h

    ; replacement
    mov [name], dword 'Nuha'           ; change the name to Nuha Ali

    ; writing the name nuha ali
    mov eax, 4
    mov ebx, 1
    mov ecx, name
    mov edx, 8              ; just enough space for Nuha Ali
    int 80h

    ;exit
    mov eax, 1
    mov ebx, 0
    int 80h
