
main:     file format elf64-x86-64


Disassembly of section .init:

000000000040033c <_init>:
  40033c:	f3 0f 1e fa          	endbr64
  400340:	48 83 ec 08          	sub    $0x8,%rsp
  400344:	48 8b 05 95 2c 00 00 	mov    0x2c95(%rip),%rax        # 402fe0 <__gmon_start__>
  40034b:	48 85 c0             	test   %rax,%rax
  40034e:	74 02                	je     400352 <_init+0x16>
  400350:	ff d0                	call   *%rax
  400352:	48 83 c4 08          	add    $0x8,%rsp
  400356:	c3                   	ret

Disassembly of section .plt:

0000000000400360 <puts@plt-0x10>:
  400360:	ff 35 8a 2c 00 00    	push   0x2c8a(%rip)        # 402ff0 <_GLOBAL_OFFSET_TABLE_+0x8>
  400366:	ff 25 8c 2c 00 00    	jmp    *0x2c8c(%rip)        # 402ff8 <_GLOBAL_OFFSET_TABLE_+0x10>
  40036c:	0f 1f 40 00          	nopl   0x0(%rax)

0000000000400370 <puts@plt>:
  400370:	ff 25 8a 2c 00 00    	jmp    *0x2c8a(%rip)        # 403000 <puts@GLIBC_2.2.5>
  400376:	68 00 00 00 00       	push   $0x0
  40037b:	e9 e0 ff ff ff       	jmp    400360 <_init+0x24>

Disassembly of section .text:

0000000000400380 <_start>:
  400380:	f3 0f 1e fa          	endbr64
  400384:	31 ed                	xor    %ebp,%ebp
  400386:	49 89 d1             	mov    %rdx,%r9
  400389:	5e                   	pop    %rsi
  40038a:	48 89 e2             	mov    %rsp,%rdx
  40038d:	48 83 e4 f0          	and    $0xfffffffffffffff0,%rsp
  400391:	50                   	push   %rax
  400392:	54                   	push   %rsp
  400393:	45 31 c0             	xor    %r8d,%r8d
  400396:	31 c9                	xor    %ecx,%ecx
  400398:	48 c7 c7 66 04 40 00 	mov    $0x400466,%rdi
  40039f:	ff 15 33 2c 00 00    	call   *0x2c33(%rip)        # 402fd8 <__libc_start_main@GLIBC_2.34>
  4003a5:	f4                   	hlt
  4003a6:	66 2e 0f 1f 84 00 00 	cs nopw 0x0(%rax,%rax,1)
  4003ad:	00 00 00 

00000000004003b0 <_dl_relocate_static_pie>:
  4003b0:	f3 0f 1e fa          	endbr64
  4003b4:	c3                   	ret
  4003b5:	66 2e 0f 1f 84 00 00 	cs nopw 0x0(%rax,%rax,1)
  4003bc:	00 00 00 
  4003bf:	90                   	nop

00000000004003c0 <deregister_tm_clones>:
  4003c0:	b8 10 30 40 00       	mov    $0x403010,%eax
  4003c5:	48 3d 10 30 40 00    	cmp    $0x403010,%rax
  4003cb:	74 13                	je     4003e0 <deregister_tm_clones+0x20>
  4003cd:	b8 00 00 00 00       	mov    $0x0,%eax
  4003d2:	48 85 c0             	test   %rax,%rax
  4003d5:	74 09                	je     4003e0 <deregister_tm_clones+0x20>
  4003d7:	bf 10 30 40 00       	mov    $0x403010,%edi
  4003dc:	ff e0                	jmp    *%rax
  4003de:	66 90                	xchg   %ax,%ax
  4003e0:	c3                   	ret
  4003e1:	0f 1f 40 00          	nopl   0x0(%rax)
  4003e5:	66 66 2e 0f 1f 84 00 	data16 cs nopw 0x0(%rax,%rax,1)
  4003ec:	00 00 00 00 

00000000004003f0 <register_tm_clones>:
  4003f0:	be 10 30 40 00       	mov    $0x403010,%esi
  4003f5:	48 81 ee 10 30 40 00 	sub    $0x403010,%rsi
  4003fc:	48 89 f0             	mov    %rsi,%rax
  4003ff:	48 c1 ee 3f          	shr    $0x3f,%rsi
  400403:	48 c1 f8 03          	sar    $0x3,%rax
  400407:	48 01 c6             	add    %rax,%rsi
  40040a:	48 d1 fe             	sar    $1,%rsi
  40040d:	74 11                	je     400420 <register_tm_clones+0x30>
  40040f:	b8 00 00 00 00       	mov    $0x0,%eax
  400414:	48 85 c0             	test   %rax,%rax
  400417:	74 07                	je     400420 <register_tm_clones+0x30>
  400419:	bf 10 30 40 00       	mov    $0x403010,%edi
  40041e:	ff e0                	jmp    *%rax
  400420:	c3                   	ret
  400421:	0f 1f 40 00          	nopl   0x0(%rax)
  400425:	66 66 2e 0f 1f 84 00 	data16 cs nopw 0x0(%rax,%rax,1)
  40042c:	00 00 00 00 

0000000000400430 <__do_global_dtors_aux>:
  400430:	f3 0f 1e fa          	endbr64
  400434:	80 3d d1 2b 00 00 00 	cmpb   $0x0,0x2bd1(%rip)        # 40300c <completed.0>
  40043b:	75 13                	jne    400450 <__do_global_dtors_aux+0x20>
  40043d:	55                   	push   %rbp
  40043e:	48 89 e5             	mov    %rsp,%rbp
  400441:	e8 7a ff ff ff       	call   4003c0 <deregister_tm_clones>
  400446:	c6 05 bf 2b 00 00 01 	movb   $0x1,0x2bbf(%rip)        # 40300c <completed.0>
  40044d:	5d                   	pop    %rbp
  40044e:	c3                   	ret
  40044f:	90                   	nop
  400450:	c3                   	ret
  400451:	0f 1f 40 00          	nopl   0x0(%rax)
  400455:	66 66 2e 0f 1f 84 00 	data16 cs nopw 0x0(%rax,%rax,1)
  40045c:	00 00 00 00 

0000000000400460 <frame_dummy>:
  400460:	f3 0f 1e fa          	endbr64
  400464:	eb 8a                	jmp    4003f0 <register_tm_clones>

0000000000400466 <main>:
  400466:	55                   	push   %rbp
  400467:	48 89 e5             	mov    %rsp,%rbp
  40046a:	bf 78 11 40 00       	mov    $0x401178,%edi
  40046f:	e8 fc fe ff ff       	call   400370 <puts@plt>
  400474:	b8 00 00 00 00       	mov    $0x0,%eax
  400479:	5d                   	pop    %rbp
  40047a:	c3                   	ret

Disassembly of section .fini:

000000000040047c <_fini>:
  40047c:	f3 0f 1e fa          	endbr64
  400480:	48 83 ec 08          	sub    $0x8,%rsp
  400484:	48 83 c4 08          	add    $0x8,%rsp
  400488:	c3                   	ret
