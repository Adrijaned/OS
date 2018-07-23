global load_idt
global idt
extern global_interrupt_handler

section .data

idt:
  times 256 dq 0
idt_end:
idtr:
  dw idt - idt_end - 1
  dd idt

section .text

load_idt:
  lidt [idtr]
  ret
