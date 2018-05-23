#![feature(lang_items)]
#![no_std]
pub mod io;

extern "C" {
    #[no_mangle]
    pub fn outb(port: u16, data: u8);
}

#[no_mangle]
pub extern fn rust_main() {
    io::_putchar(35);
}

#[lang = "eh_personality"] #[no_mangle] pub extern fn eh_personality() {}
#[lang = "panic_fmt"] #[no_mangle] pub extern fn panic_fmt() -> ! {loop{}}