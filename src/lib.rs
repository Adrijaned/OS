#![feature(lang_items)]
#![feature(panic_implementation)]
#![no_std]

#[macro_use]
pub mod io;

use core::panic::PanicInfo;

extern "C" {
    #[no_mangle]
    pub fn outb(port: u16, data: u8);

    #[no_mangle]
    pub fn inb(port: u16) -> u8;
}

#[no_mangle]
pub extern fn rust_main() {
    io::_putchar(12); // clear screen
    println!("Hello!")
}

#[lang = "eh_personality"]
#[no_mangle]
pub extern fn eh_personality() {}

#[panic_implementation]
#[no_mangle]
pub extern fn panic_fmt(_info: &PanicInfo) -> ! { loop {} }