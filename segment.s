global load_gdt
extern _putchar

load_gdt:
    cli             ; Disable interrupts just to make sure
    pusha
    lgdt [gdtr]
    popa
    jmp 0x08:.end   ; to set cs
.end:
    push ax
    mov ax, 0x10
    mov ds, ax      ; sector registers can't be set directly
    mov ss, ax
    mov es, ax
    mov gs, ax
    mov fs, ax
    pop ax
    ret

gdt:    ; Global Descriptor Table
        ; base - where the segment starts
        ; limit - size of the segment
  .null_sector: ; required
    dq 0
  .code_sector:
    dw 0xFFFF   ; limit 0:15
    dw 0        ; base 0:15
    db 0        ; base 16:23
    db 10011010B; Access byte:
                ; bit 7 - "Present" flag - 1 for all valid segment selectors
                ; bits 6:5 - Privilege selector - based on this in the currently
                ;   selected segment selector CPU operates in the protection ring
                ; bit 4 - descriptor type - 1 for code/data segments
                ; bit 3 - 1 for code segment, 0 for data segment
                ; bit 2 - 0 (empirically tested)
                ; bit 1 - 1 for read access to code segment/write access to data
                ;   segment instead of just execute/read
                ; bit 0 - Set to 0, processor handles by itself
    db 11001111B
                ; bit 7 - "Granularity" bit - 1 for limit to be interpreted in
                ;   4kiB blocks, 0 to interpret in bytes
                ; bit 6 - "size" bit - 1 for 32bit mode segment selectors, 0 for 16bit
                ; bits 5:4 - 0; reserved for future use
                ; bits 3:0 - limit 16:19
    db 0        ; base 24:31
  .data_sector:
    dw 0xFFFF
    dw 0
    db 0
    db 10010010B
    db 11001111B
    db 0

gdtr:   ; GDT descriptor
    dw gdtr - gdt - 1   ; size of GDT - 1
    dd gdt              ; Address of GDT
