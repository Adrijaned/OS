const PIC1_CMD: u16 = 0x20;
const PIC1_DATA: u16 = 0x21;
const PIC2_CMD: u16 = 0xA0;
const PIC2_DATA: u16 = 0xA1;

/// Initializes standard 8259a PIC.
/// [source](https://pdos.csail.mit.edu/6.828/2014/readings/hardware/8259A.pdf)
/// [source2](https://wiki.osdev.org/PIC)
pub unsafe fn init_pic(offset1: u8, offset2: u8) {
    // save old masks
    let mask1 = ::inb(PIC1_DATA);
    let mask2 = ::inb(PIC2_DATA);

    // | 7                 5 | 4 |         3         |         2         |             1            |        0        |
    // | null on non-MS80/85 | 1 | 0 on standard x86 | 0 on standard x86 | cascade mode, not single | ICW4 is needed  |
    let icw1 = 0b_0001_0001u8;

    // ICW2 is offset

    // Tells where all is slave, bit 0 corresponds to IRQ0, bit 7 to IRQ7, this is slave on IRQ2
    let icw3_master = 0b_000_0100u8;

    // Tells slave where it is on master, 0x0 corresponds to IRQ0, 0x7 to IRQ7, 0x2 to IRQ2
    let icw3_slave = 2u8;

    // | 7   5 |            4             |       3      |                 2           |      1     |               0             |
    // | 0 0 0 | not special fully nested | non-buffered | non-buffered, so not-needed | normal eoi | 8086/8088 mode, not MS-80/85|
    let icw4 = 0b_0000_0001u8;

    // send config bytes to each of PICs
    ::outb(PIC1_CMD, icw1);
    wait!();
    ::outb(PIC2_CMD, icw1);
    wait!();
    ::outb(PIC1_DATA, offset1);
    wait!();
    ::outb(PIC2_DATA, offset2);
    wait!();
    ::outb(PIC1_DATA, icw3_master);
    wait!();
    ::outb(PIC2_DATA, icw3_slave);
    wait!();
    ::outb(PIC1_DATA, icw4);
    wait!();
    ::outb(PIC2_DATA, icw4);
    wait!();

    // restore old masks
    ::outb(PIC1_DATA, mask1);
    ::outb(PIC2_DATA, mask2);
}

pub unsafe fn send_eoi(irq: u8) {
    // EOI command code is 0x20

    // EOI needs to be sent to slave to if applicable
    if irq >= 8 {
        ::outb(PIC2_CMD, 0x20u8);
    }
    ::outb(PIC1_CMD, 0x20u8);
}

pub unsafe fn set_irq_flag(irq: u8) {
    let port = if irq >= 8 { PIC2_DATA } else { PIC1_DATA };
    ::outb(port, ::inb(port) | 1 << (irq % 8))
}

pub unsafe fn clear_irq_flag(irq: u8) {
    let port = if irq >= 8 { PIC2_DATA } else { PIC1_DATA };
    ::outb(port, ::inb(port) & (0b_1111_1111u8 ^ (1 << (irq % 8)))) // ^ - bitwise xor
}