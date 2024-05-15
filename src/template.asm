section .bss
    buffer: resb 30000

section .text
    global _start

%macro get 0
    lea r9, [buffer + r8]
    mov rax, 0
    mov rdi, 0
    mov rsi, r9
    mov rdx, 1
    syscall
%endmacro

%macro put 0
    lea r9, [buffer + r8]
    mov rax, 1
    mov rdi, 1
    mov rsi, r9
    mov rdx, 1
    syscall
%endmacro

_start:
    mov r8, 0
;   code
    mov rax, 60
    mov rdi, 0
    syscall
