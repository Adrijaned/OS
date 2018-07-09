global load_idt
global idt

idt:
  times 256 dq 0
idt_end:
idtr:
  dw idt - idt_end - 1
  dd idt

load_idt:
  lidt [idtr]
  ret