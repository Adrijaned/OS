#![feature(lang_items)]
#![feature(asm)]
#![feature(naked_functions)]
#![no_std]

macro_rules! wait {
    () => {::outb(0x80, 10)}; // random data on unused port - IO to delay processor
}

#[macro_use]
pub mod io;
pub mod interrupts;
pub mod bootloader;
pub mod time;

pub use interrupts::*;
use core::panic::PanicInfo;

extern "C" {
    #[no_mangle]
    pub fn outb(port: u16, data: u8);

    #[no_mangle]
    pub fn inb(port: u16) -> u8;

    #[no_mangle]
    pub fn load_gdt();
}

#[no_mangle]
pub unsafe extern fn rust_main(idt: *mut interrupts::IdtEntry, ebx: *const bootloader::Multiboot1Structure, eax: u32) {
    load_gdt();
    io::_putchar(12); // clear screen
    io::status_bar::init();
    println!("Hello!");
    time::init();

    interrupts::init(idt as u32, 0x08);

    if eax == 0x2BADB002 {
        println!("Compliance with Multiboot1 confirmed.");
        unsafe {
            match (*ebx).mmap_addr() {
                Ok(_x) => {
//                    println!(core::ptr::read(x as *const bootloader::MmapEntry))
                }
                Err(_) => {
                    println!("Fatal exception, could not retrieve memory map")
                }
            }
        }
    } else {
        println!("Could not verify compliance with Multiboot1")
    }
}

#[lang = "eh_personality"]
#[no_mangle]
pub extern fn eh_personality() {}

#[panic_handler]
#[no_mangle]
pub extern fn panic_fmt(_info: &PanicInfo) -> ! { loop {} }