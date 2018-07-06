#![feature(lang_items)]
#![feature(panic_implementation)]
#![no_std]

#[macro_use]
pub mod io;

pub mod bootloader;

use core::panic::PanicInfo;

extern "C" {
    #[no_mangle]
    pub fn outb(port: u16, data: u8);

    #[no_mangle]
    pub fn inb(port: u16) -> u8;
}

#[no_mangle]
pub extern fn rust_main(eax: u32, ebx: u32) {
    io::_putchar(12); // clear screen
    println!("Hello!");
    if eax == 0x2BADB002 {println!("Presence of Multiboot1 confirmed.")}
    match unsafe { (*(ebx as *mut bootloader::Multiboot1Structure)).mem_lower() } {
        Ok(x) => {
            println!(x);
        }
        Err(_) => {
            println!("Multiboot flag 0 clear, could not probe memory");
        }
    }
}

#[lang = "eh_personality"]
#[no_mangle]
pub extern fn eh_personality() {}

#[panic_implementation]
#[no_mangle]
pub extern fn panic_fmt(_info: &PanicInfo) -> ! { loop {} }