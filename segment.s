global load_gdt
extern _putchar

load_gdt:
    cli
    pusha
    lgdt [gdtr]
    popa
    jmp 0x08:.end
.end:
    push ax
    mov ax, 0x10
    mov ds, ax
    mov ss, ax
    mov es, ax
    mov gs, ax
    mov fs, ax
    pop ax
    ret

gdt:
  .null_sector:
    dq 0
  .code_sector:
    dw 0xFFFF
    dw 0
    db 0
    db 10011010B
    db 11001111B
    db 0
  .data_sector:
    dw 0xFFFF
    dw 0
    db 0
    db 10010010B
    db 11001111B
    db 0

gdtr:
    dw gdtr - gdt - 1
    dd gdt
