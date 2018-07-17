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
    println!(1u32);
    for x in 0..256 {
        (*((first_entry + x * 8) as *mut IdtEntry)).set_selector(selector);
        (*((first_entry + x * 8) as *mut IdtEntry)).set_offset(other_exception as extern "C" fn() as u32);
        (*((first_entry + x * 8) as *mut IdtEntry)).set_flags_hardware();
    }
    (*((first_entry + 13 * 8) as *mut IdtEntry)).set_offset(other_exception2 as extern "C" fn() as u32);
    (*((first_entry + 8 * 8) as *mut IdtEntry)).set_offset(other_exception3 as extern "C" fn() as u32);
    (*((first_entry + 8 * 8) as *mut IdtEntry)).disable();
    (*((first_entry) as *mut IdtEntry)).set_offset(div_by_zero as extern "C" fn() as u32);
    println!(div_by_zero as extern "C" fn() as u32);
    println!((*(first_entry as *const IdtEntry)).offset_lower);
    println!((*(first_entry as *const IdtEntry)).offset_higher);
//    cli();
    asm!("call load_idt"::::"intel","volatile");
    super::io::putchar('a');
    sti();
}

#[inline(always)]
fn cli() {
    unsafe {
        asm!("cli"::::"intel","volatile")
    }
}

#[inline(always)]
fn sti() {
    unsafe {
        asm!("sti"::::"intel","volatile");
    }
}

#[no_mangle]
pub extern "C" fn div_by_zero() {
    unsafe {
        asm!("pusha"::::"intel","volatile")
    }
//    cli();
    println!("divbyzero");
//    sti();
    unsafe {
        asm!("popa"::::"intel","volatile");
        asm!("mov ecx, 1"::::"intel","volatile");
        asm!("iret"::::"intel","volatile");
    }
}

#[no_mangle]
pub extern "C" fn other_exception() {
    unsafe {
        asm!("pusha"::::"intel","volatile");
    }
    super::io::putchar('x');
    unsafe {
        asm!("popa"::::"intel","volatile");
        asm!("iret"::::"intel","volatile");
    }
}

#[no_mangle]
pub extern "C" fn other_exception3() {
    unsafe {
        asm!("pusha"::::"intel","volatile");
        let cr2: u32;
        asm!("mov eax, cr2" : "={eax}"(cr2) : : : "intel", "volatile");
        print!("Page fault: ");
        println!(cr2);
        asm!("popa"::::"intel","volatile");
        asm!("iret"::::"intel","volatile");
    }
}

#[no_mangle]
pub extern "C" fn other_exception2() {
    unsafe {
        asm!("pusha"::::"intel","volatile");
        {
            let c: u32;
            asm!("mov eax, [esp + 32]":"={eax}"(c):::"intel","volatile");
//            println!(c);
        }
        asm!("popa"::::"intel","volatile");
        asm!("add esp, 4"::::"intel","volatile");
        asm!("iret"::::"intel","volatile");
    }
}