## Syntax
- Registers start with `%`. Eg: `%rax` - the register `rax`
- Immediates start with `$`. Eg: `$42` - the literal number 42
- Operand order is **src**, **dst**. Eg: `movq %rdi, %rax` - equals to `rax = rdi`
- Mnemonics get a size suffix. Eg: `movq` - `q` = quardword = 64 bits

## Size suffixes
| Suffix    | Size (bytes) |
|-----------|--------------|
| b (byte)  | 8b           |
| w (word)  | 16b          |
| l (long)  | 32b          |
| q (quad)  | 64b          |


## Registers
- x86_64 has 16 general purpose 64-bit registers
- first 8 are ancient and names for historical purposes

| 64-bit | 32-bit | 16-bit | 8-bit | Historial role                 |
|--------|--------|--------|-------|--------------------------------|
| rax    | eax    | ax     | al    | accumulator(return value)      |
| rbx    | ebx    | bx     | bl    | base                           |
| rcx    | ecx    | cx     | cl    | counter (loop/shl)             |
| rdx    | edx    | dx     | dl    | data (2nd return value)        |
| rsi    | esi    | si     | sil   | source index (2nd arg)         |
| rdi    | edi    | di     | dil   | destination index (1st arg)    |
| rbp    | ebp    | bp     | bpl   | base pointer (frame pointer)   |
| rsp    | esp    | sp     | spl   | stack pointer                  |
| r8-r15 | r8d..  | r8w..  | r8b.. | added by AMD in 2003           |
