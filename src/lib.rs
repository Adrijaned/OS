#![feature(lang_items)]
#![no_std]

pub mod io;

extern "C" {
    #[no_mangle]
    pub fn outb(port: u16, data: u8);

    #[no_mangle]
    pub fn inb(port: u16) -> u8;
}

#[no_mangle]
pub extern fn rust_main() {
    unsafe {
        outb(0x3d8, 0b_1111_1011);
        io::_putchar(12); // clear screen
        io::print("Howdy buddies!")
    }
}

#[lang = "eh_personality"]
#[no_mangle]
pub extern fn eh_personality() {}

#[lang = "panic_fmt"]
#[no_mangle]
pub extern fn panic_fmt() -> ! { loop {} }