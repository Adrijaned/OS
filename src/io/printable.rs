use super::{_putchar, putchar};

pub trait Printable {
    fn vga_print(&self);
}

impl Printable for (u8, u8, u8) {
    fn vga_print(&self) {
        _putchar(self.0);
        _putchar(self.1);
        _putchar(self.2);
    }
}

impl Printable for (u8, u8) {
    fn vga_print(&self) {
        _putchar(self.0);
        _putchar(self.1);
    }
}

impl Printable for (char, char) {
    fn vga_print(&self) {
        putchar(self.0);
        putchar(self.1);
    }
}

impl<'a> Printable for &'a str {
    fn vga_print(&self) {
        for x in self.as_bytes() {
            _putchar(*x);
        }
    }
}

impl Printable for u32 {
    fn vga_print(&self) {
        ('0', 'x').vga_print();
        super::typecast::u8_to_hex_ascii(((self >> 24) & 0xffu32) as u8).vga_print();
        super::typecast::u8_to_hex_ascii(((self >> 16) & 0xffu32) as u8).vga_print();
        super::typecast::u8_to_hex_ascii(((self >> 8) & 0xffu32) as u8).vga_print();
        super::typecast::u8_to_hex_ascii((self & 0xffu32) as u8).vga_print();
    }
}