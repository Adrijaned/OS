global loader
extern kmain, idt, div_by_zero, _print_int

MAGIC_NUM 	equ 		0x1BADB002
FLAGS 		equ 		0x0
CHECKSUM 	equ 		-MAGIC_NUM

section .text
align 4
	dd		MAGIC_NUM
	dd		FLAGS
	dd		CHECKSUM

loader:
    mov esp, stack_start+4096
	push eax
	push ebx
	push idt
	call        kmain
.loop:
	jmp		.loop

section .bss
align 4
stack_start:
resb 4096
