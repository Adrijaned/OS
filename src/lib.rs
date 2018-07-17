#![feature(lang_items)]
#![feature(panic_implementation)]
#![feature(asm)]
#![no_std]

#[macro_use]
pub mod io;
pub mod interrupts;
pub mod bootloader;

pub use interrupts::*;
use core::panic::PanicInfo;

extern "C" {
    #[no_mangle]
    pub fn outb(port: u16, data: u8);

    #[no_mangle]
    pub fn inb(port: u16) -> u8;
}

#[no_mangle]
pub extern fn rust_main(eax: u32, ebx: *const bootloader::Multiboot1Structure, idt: *mut interrupts::IdtEntry) {
    io::_putchar(12); // clear screen
    println!("Hello!");

    unsafe {
        interrupts::init(idt as u32, 0x08);
    }

    if eax == 0x2BADB002 {println!("Compliance with Multiboot1 confirmed.")}
    unsafe {
        match (*ebx).mmap_addr() {
            Ok(x) => {
                println!(core::ptr::read(x as *const bootloader::MmapEntry))
            }
            Err(_) => {
                println!("Fatal exception, could not retrieve memory map")
            }
        }
    }
}

#[lang = "eh_personality"]
#[no_mangle]
pub extern fn eh_personality() {}

#[panic_implementation]
#[no_mangle]
pub extern fn panic_fmt(_info: &PanicInfo) -> ! { loop {} }