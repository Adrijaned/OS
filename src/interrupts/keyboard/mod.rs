mod keymap;
pub unsafe fn handle_irq(key_code: u8){
    ///
    let is_make = key_code & 0b_1000__0000 == 0;
    let key_code = key_code & 0b_0111_1111;
    if key_code == 0xe0 {
        wait!();
        let key_code = ::inb(0x60);
        E0_SCANCODES[(key_code) as usize] = is_make;
    }
    BASE_SCANCODES[(key_code) as usize] = is_make;
    if is_make {
        match keymap::get_char(key_code, false, 1) {
            Err(_) => println!("Keyboard error"),
            Ok(Some(x)) => ::io::input_buffer::register_keypress(x),
            Ok(_) => (),
        }
    }
}

static mut BASE_SCANCODES: [bool; 0x80] = [false; 0x80];
static mut E0_SCANCODES:  [bool; 0x80] = [false; 0x80];