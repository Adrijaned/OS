const MEMORY_START: u32 = super::VGA_FRAMEBUFFER_MEMORY_START + 3840;

pub fn init() {
    let mut mem = MEMORY_START + 1;
    for _ in 1..=80 {
        unsafe { *(mem as *mut u8)  = 0b_0001_1010}
        mem += 2;
    }
}

pub fn set_time(hours: u8, minutes: u8, seconds: u8) {
    unsafe{
        *(MEMORY_START as *mut u8) = hours / 10 + 0x30u8;
        *((MEMORY_START + 2) as *mut u8) = (hours % 10) + 0x30u8;
        *((MEMORY_START + 4) as *mut u8) = 0x3au8;
        *((MEMORY_START + 6) as *mut u8) = minutes / 10 + 0x30u8;
        *((MEMORY_START + 8) as *mut u8) = (minutes % 10) + 0x30u8;
        *((MEMORY_START + 10) as *mut u8) = 0x3au8;
        *((MEMORY_START + 12) as *mut u8) = seconds / 10 + 0x30u8;
        *((MEMORY_START + 14) as *mut u8) = (seconds % 10) + 0x30u8;
    }
}

pub fn beep(frequency: u16) {
    unsafe {
        *((MEMORY_START + 152) as *mut u8) = 0x42;
        *((MEMORY_START + 154) as *mut u8) = 0x45;
        *((MEMORY_START + 156) as *mut u8) = 0x45;
        *((MEMORY_START + 158) as *mut u8) = 0x50;
        ::interrupts::beep(frequency)
    }
}
pub fn no_beep() {
    unsafe {
        *((MEMORY_START + 152) as *mut u8) = 0;
        *((MEMORY_START + 154) as *mut u8) = 0;
        *((MEMORY_START + 156) as *mut u8) = 0;
        *((MEMORY_START + 158) as *mut u8) = 0;
        ::interrupts::no_beep();
    }
}
