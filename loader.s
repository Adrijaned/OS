global loader
extern kmain, idt

MAGIC_NUM 	equ 		0x1BADB002
FLAGS 		equ 		0x0
CHECKSUM 	equ 		-MAGIC_NUM

section .text
align 4
	dd		MAGIC_NUM
	dd		FLAGS
	dd		CHECKSUM

loader:
	push eax
	push ebx
	push idt
	call        kmain
.loop:
	jmp		.loop
