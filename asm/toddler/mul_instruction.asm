section	.text
   global _start    ;must be declared for using gcc
	
_start:             ;tell linker entry point

   mov	al,'3'
   sub     al, '0'
	
   mov 	bl, '2'
   sub     bl, '0'
   mul 	bl
   add	al, '0'
	
   mov 	[res], al
   mov	ecx,msg	
   mov	edx, len
   mov	ebx,1	;file descriptor (stdout)
   mov	eax,4	;system call number (sys_write)
   int	0x80	;call kernel
	
   mov	ecx,res
   mov	edx, 1
   mov	ebx,1	;file descriptor (stdout)
   mov	eax,4	;system call number (sys_write)
   int	0x80	;call kernel

   ; perhaps a new line
   mov eax, 4
   mov ebx, 1
   mov ecx, nxt_line
   mov edx, 1
   int 80h
	
   mov	eax,1	;system call number (sys_exit)
   int	0x80	;call kernel

section .data
    msg db "The result is:", 0xA,0xD 
    len equ $- msg
    nxt_line db '', 10
segment .bss
    res resb 1
