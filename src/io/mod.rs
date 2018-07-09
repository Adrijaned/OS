const VGA_FRAMEBUFFER_MEMORY_START: u32 = 0x000B8000;
static mut VGA_FRAMEBUFFER_LINE: u16 = 0;
static mut VGA_FRAMEBUFFER_CHAR: u16 = 0;

pub mod typecast;

mod printable;
pub use self::printable::Printable;

macro_rules! println {
    () => {$crate::io::print("\n")};
    ($fmt:expr) => {{
        $crate::io::print($fmt);
        $crate::io::print("\n")
    }}
}

macro_rules! print {
    () => {};
    ($fmt:expr) => {{
        $crate::io::print($fmt)
    }}
}

pub fn print(string: impl Printable) {
    string.vga_print()
}
/// Writes a character on the place of cursor.
///
/// Moves cursor.
pub fn putchar(character: char) {
    _putchar(character as u8)
}

/// Writes a character, specified by its ascii code, to the place of the cursor.
///
/// Moves cursor.
#[no_mangle]
pub extern fn _putchar(character: u8) {
    unsafe {
        match character {
            8 if VGA_FRAMEBUFFER_LINE != 0 => VGA_FRAMEBUFFER_LINE -= 1,
            9 => {
                *(get_memory_offset() as *mut u64) = 0;
                VGA_FRAMEBUFFER_CHAR += 4
            }
            10 => {
                VGA_FRAMEBUFFER_LINE += 1;
                VGA_FRAMEBUFFER_CHAR = 0;
            }
            12 => clear(),
            32 => VGA_FRAMEBUFFER_CHAR += 1,
            33...126 => {
                *(get_memory_offset()) = (character as u16) + 0b_0000_1111_0000_0000; // little endian system
                VGA_FRAMEBUFFER_CHAR += 1;
            }
            _ => (),
        }
        if VGA_FRAMEBUFFER_CHAR >= 80 {
            VGA_FRAMEBUFFER_CHAR -= 80;
            VGA_FRAMEBUFFER_LINE += 1
        }
        if VGA_FRAMEBUFFER_LINE >= 25 {
            scroll()
        }
        move_cursor_to_current();
    }
}

unsafe fn move_cursor_to_current() {
    let pos: u16 = 80 * VGA_FRAMEBUFFER_LINE + VGA_FRAMEBUFFER_CHAR;
    super::outb(0x3D4, 14);
    super::outb(0x3D5, ((pos >> 8) as u8) & 0xFF);
    super::outb(0x3D4, 15);
    super::outb(0x3D5, (pos as u8) & 0xFF);
}

/// Calculates correct memory address from line and column, for vga framebuffer memory character..
unsafe fn get_memory_offset() -> *mut u16 {
    ((2 * (VGA_FRAMEBUFFER_LINE * 80 + VGA_FRAMEBUFFER_CHAR)) as u32 + VGA_FRAMEBUFFER_MEMORY_START) as *mut u16
}

/// Clears whole screen.
///
/// Also sets the active point to top-left corner.
unsafe fn clear() {
    VGA_FRAMEBUFFER_CHAR = 0;
    VGA_FRAMEBUFFER_LINE = 0;
    let mut mem = VGA_FRAMEBUFFER_MEMORY_START;
    for _ in 1..=2000 {
        *(mem as *mut u16) = 0;
        mem += 2;
    }
}

/// Scrolls whole screen up.
///
/// Also moves the active point line up.
unsafe fn scroll() {
    if VGA_FRAMEBUFFER_LINE != 0 {
        VGA_FRAMEBUFFER_LINE -= 1;
    }
    let mut mem = VGA_FRAMEBUFFER_MEMORY_START;
    for _ in 1..=1920 {
        *(mem as *mut u16) = *((mem + 160) as *const u16);
        mem += 2
    }
    for _ in 1..=80 {
        *(mem as *mut u16) = 0;
        mem += 2;
    }
}
