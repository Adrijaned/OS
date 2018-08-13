#[naked]
#[no_mangle]
pub extern "C" fn div_by_zero() {
    unsafe {
        asm!("pusha"::::"intel","volatile");
        asm!("push 'd'"::::"intel","volatile");
        asm!("call _putchar"::::"intel","volatile");
        asm!("add esp, 4"::::"intel","volatile");
        asm!("popa"::::"intel","volatile");
        asm!("mov ecx, 2"::::"intel","volatile");
        asm!("iretd"::::"intel","volatile");
    }
}

#[naked]
#[no_mangle]
pub extern "C" fn other_exception() {
    unsafe {
        asm!("pusha"::::"intel","volatile");
        asm!("push 'x'"::::"intel","volatile");
        asm!("call _putchar"::::"intel","volatile");
        asm!("add esp, 4"::::"intel","volatile");
        asm!("popa"::::"intel","volatile");
        asm!("iretd"::::"intel","volatile");
    }
}

#[naked]
#[no_mangle]
pub extern "C" fn general_protection_fault() {
    unsafe {
        asm!("pusha"::::"intel","volatile");
        asm!("mov eax, [esp + 32]"::::"intel", "volatile");
        asm!("push 'g'"::::"intel","volatile");
        asm!("call _putchar"::::"intel","volatile");
        asm!("add esp, 4"::::"intel","volatile");
        asm!("push eax"::::"intel","volatile");
        asm!("call _print_int"::::"intel","volatile");
        asm!("add esp, 4"::::"intel","volatile");
        asm!("popa"::::"intel","volatile");
        asm!("add esp, 4"::::"intel","volatile");
        asm!("iretd"::::"intel","volatile");
    }
}

#[naked]
#[no_mangle]
pub extern "C" fn double_fault() {
    unsafe {
        asm!("pusha"::::"intel","volatile");
        asm!("mov eax, [esp + 32]"::::"intel","volatile");
        asm!("push eax"::::"intel","volatile");
        asm!("call _print_int");
        asm!("add esp, 4"::::"intel","volatile");
        asm!("popa"::::"intel","volatile");
        asm!("iretd"::::"intel","volatile");
    }
}

#[naked]
#[no_mangle]
pub extern "C" fn timer_irq() {
    #[inline(never)]
    unsafe fn inner() {
        ::time::tick();
        super::pic::send_eoi(0);
    }
    unsafe {
        asm!("pusha"::::"intel","volatile");
        inner();
        asm!("popa"::::"intel","volatile");
        asm!("iretd"::::"intel","volatile");
    }
}
