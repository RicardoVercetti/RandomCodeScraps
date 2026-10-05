// commands
// 1. gcc -O0 -S hands_on.c -o add.s
// 2. gcc -O2 -S hands_on.c -o add2.s
// 3. objdump -d call.o
// 4. gcc -O0 -c main.c -o main.o                       # -c → just compile, don't link
// 5. nm main.o                                         # T → defined here, U → undefined, needs to be resolved
// 6. gcc -shared -fPIC -O2 add.c -o libadd.so          # 

// questions:
// i. whats the difference between .o file and .s file
// ii. what does the -d flag do in the objdump command?
// iii. output asm syntax for first -O0 flag command

// Exercises:
// i. Write long sub(long a, long b) and disassemble it. Confirm a is in rdi, b in rsi.
// ii. Write a function with 8 int args, disassemble, find where arg #7 and #8 live on the stack.
// iii. Write a function returning a struct { long a; long b; } and disassemble — you'll see the return goes in rax and rdx (or via hidden pointer if bigger).
// iv. strace -f -e trace=write ./hello on a hello-world binary and correlate the syscall registers with the syscall(2) man page.
// v. Compile something with -O2 and try to identify the callee-saved prologue (push rbx / push r12 ...) and matching epilogue.

#include <stdio.h>

long add(long, long);

int main(void) {
    long value = add(3, 4);
    printf("string returned: %ld\n", value);
    return 0;
}
