// (programmable) (interrupt) timer. Used mainly for beeping with beeper. Raises IRQ0 whenever the signal is issued


const CLOCK_RATE: f64 = 105_000_000f64 / 88f64; //wikipedia constant

pub unsafe fn set_frequency(micros: u16) -> i32{
    let hertz_rate = 1_000_000f64 / (micros as f64);
    let clock_ticks = (CLOCK_RATE / hertz_rate) as u16;
    ::outb(0x43u16, 0b_0011_0100); // channel 0 (00), send lobyte/hibyte (11), mode 2 (010), binary, not BCD (0)
    ::outb(0x40u16, (clock_ticks & 0xFF) as u8); // send lobyte to channel 0 port
    ::outb(0x40u16, ((clock_ticks >> 8) & 0xFF) as u8); // send hibyte to channel 0 port
    - ((micros as f64 - (88f64 * clock_ticks as f64) / 105f64) * 1_000_000f64) as i32 //picos error. When added to micros passed in, they equal the accurate time between each two ticks
}

pub unsafe fn beep(frequency: u16) {
    let clock_ticks = (CLOCK_RATE / frequency as f64) as u16;
    ::outb(0x43u16, 0b_1011_0110); // channel 2 (10), send lobyte/hibyte (11), mode 3 (013), binary, not BCD (0)
    ::outb(0x42u16, (clock_ticks & 0xFF) as u8);
    ::outb(0x42u16, ((clock_ticks >> 8) & 0xFF) as u8);

    // enable speaker, link to channel 2
    let tmp = ::inb(0x61u16);
    if tmp != tmp | 3 {
        ::outb(0x61u16, tmp | 3)
    }
}

pub unsafe fn no_beep() {
    let tmp = ::inb(0x61u16);
    ::outb(0x61u16, tmp & 0b_1111_1100)
}