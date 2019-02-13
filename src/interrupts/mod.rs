pub use self::pit::set_frequency;
pub use self::pit::{beep, no_beep};
mod pic;
mod pit;
mod keyboard;

#[repr(C,packed)]
pub struct IdtEntry {
    offset_lower: u16,
    segment_selector: u16,
    zero: u8,
    type_attr: u8,
    offset_higher: u16,
}

impl IdtEntry {
    pub fn set_offset(&mut self, offset: u32) {
        self.offset_lower = (offset & 0xFFFF) as u16;
        self.offset_higher = ((offset >> 16) & 0xFFFF) as u16;
    }

    pub fn set_selector(&mut self, selector: u16) {
        self.segment_selector = selector;
    }

    pub fn set_flags_hardware(&mut self) {
        self.type_attr = 0b_1000_1110;
        self.zero = 0;
    }

    pub fn disable(&mut self) {
        self.offset_higher = 0;
        self.offset_lower = 0;
        self.zero = 0;
        self.type_attr = 0;
        self.segment_selector = 0;
    }
}

pub unsafe fn init(first_entry: u32, selector: u16) {
    cli();
    for x in 0..256 {
        (*((first_entry + x * 8) as *mut IdtEntry)).set_selector(selector);
        (*((first_entry + x * 8) as *mut IdtEntry)).set_offset(handlers::other_exception as extern "C" fn() as u32);
        (*((first_entry + x * 8) as *mut IdtEntry)).set_flags_hardware();
    }
    (*((first_entry + 13 * 8) as *mut IdtEntry)).set_offset(handlers::general_protection_fault as extern "C" fn() as u32);
    (*((first_entry + 8 * 8) as *mut IdtEntry)).set_offset(handlers::double_fault as extern "C" fn() as u32);
    (*((first_entry) as *mut IdtEntry)).set_offset(handlers::div_by_zero as extern "C" fn() as u32);
    let first_irq_entry = first_entry + 0x20 * 8;
    (*(first_irq_entry as *mut IdtEntry)).set_offset(handlers::timer_irq as extern "C" fn() as u32);
    (*((first_irq_entry + 1 * 8) as *mut IdtEntry)).set_offset(handlers::keyboard_irq as extern "C" fn() as u32);
    asm!("call load_idt"::::"intel","volatile");
    pic::init_pic(0x20, 0x28); // put them right after CPU exceptions
    sti();
}

#[inline(always)]
#[naked]
fn cli() {
    unsafe {
        asm!("cli"::::"intel","volatile")
    }
}

#[naked]
#[inline(always)]
fn sti() {
    unsafe {
        asm!("sti"::::"intel","volatile");
    }
}

pub mod handlers;
