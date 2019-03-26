const RESOLUTION_MICROS: u16 = 50_000u16;

static mut ERROR_PICOS: i32 = 0;

static mut TIMESTAMP: u64 = 0;
static mut TIMESTAMP_SINCE_BOOTUP: u64 = 0;
static mut CURRENT_MICROS: u32 = 0;
static mut CURRENT_ERROR_PICOS: i32 = 0;

pub unsafe fn tick() {
    CURRENT_MICROS += RESOLUTION_MICROS as u32;
    CURRENT_ERROR_PICOS += ERROR_PICOS;
    if CURRENT_ERROR_PICOS >= 1_000_000 {
        CURRENT_ERROR_PICOS -= 1_000_000;
        CURRENT_MICROS += 1;
    } else if CURRENT_ERROR_PICOS <= -1_000_000 {
        CURRENT_ERROR_PICOS += 1_000_000;
        CURRENT_MICROS -= 1;
    }
    if CURRENT_MICROS >= 1_000_000 {
        CURRENT_MICROS -= 1_000_000;
        TIMESTAMP += 1;
        TIMESTAMP_SINCE_BOOTUP += 1;
        ::io::status_bar::set_time(((TIMESTAMP % 86400) / 3600) as u8,((TIMESTAMP % 3600) / 60) as u8,(TIMESTAMP % 60) as u8)
    }
}

pub fn get_timestamp() -> u64 {
    unsafe {
        TIMESTAMP
    }
}

pub (super) fn init() {
    unsafe {
        ERROR_PICOS = ::interrupts::set_frequency(RESOLUTION_MICROS);
        let nmi_bit = ::inb(0x70) & 0b_1000_0000;
        'retrieve_time_main: loop {
            TIMESTAMP_SINCE_BOOTUP = 0;
            TIMESTAMP = 0;
            while {::outb(0x70, 0x0A & nmi_bit); ::inb(0x71) & 0b_1000_0000 > 0} { wait!() }
            ::outb(0x70, 0x00 | nmi_bit);
            TIMESTAMP_SINCE_BOOTUP += ::inb(0x71) as u64;
            ::outb(0x70, 0x02 | nmi_bit);
            TIMESTAMP_SINCE_BOOTUP += ::inb(0x71) as u64 * 60;
            ::outb(0x70, 0x04 | nmi_bit);
            TIMESTAMP_SINCE_BOOTUP += ::inb(0x71) as u64 * 3600;
            while {::outb(0x70, 0x0A & nmi_bit); ::inb(0x71) & 0b_1000_0000 > 0} { wait!() }
            ::outb(0x70, 0x00 | nmi_bit);
            let secs = ::inb(0x71) as u64;
            TIMESTAMP += secs;
            ::outb(0x70, 0x02 | nmi_bit);
            let mins = ::inb(0x71) as u64;
            TIMESTAMP += mins * 60;
            ::outb(0x70, 0x04 | nmi_bit);
            let hrs = ::inb(0x71) as u64;
            TIMESTAMP += hrs * 3600;
            if TIMESTAMP == TIMESTAMP_SINCE_BOOTUP {
                TIMESTAMP_SINCE_BOOTUP = 0;
                ::outb(0x70, 0x0b | nmi_bit);
                let status_byte_b = ::inb(0x71);
                let is_bcd = (status_byte_b & 0b_0000_0100) == 0;
                let is_12h = (status_byte_b & 0b_0000_0010) == 0;
                let is_pm = hrs & 0b_1000_0000 > 0;
                let (secs, mins, hrs) = if is_bcd {
                    (
                        ((secs / 16) * 10) + (secs & 0xf),
                        ((mins / 16) * 10) + (mins & 0xf),
                        (((hrs  / 16) * 10) + (hrs  & 0xf)) & 0b_0111_1111
                    )
                } else { (secs, mins, hrs) };
                let hrs = if is_pm { hrs + 12 } else { hrs };
                let hrs = if hrs % 12 == 0 && is_12h { hrs - 12 } else { hrs };
                TIMESTAMP = secs + 60*mins + 3600*hrs;
                println!(secs as u16);
                println!(mins as u16);
                println!(hrs as u16);
                break 'retrieve_time_main;
            }
        }
        println!(TIMESTAMP);
    }
}

