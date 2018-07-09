pub fn u8_to_dec_ascii(character: u8) -> (u8, u8, u8) {
    ((character / 100) + 0x30, ((character / 10) % 10) + 0x30, (character % 10) + 0x30)
}

pub fn u8_to_hex_ascii(character: u8) -> (u8, u8) {
    let smaller = character % 16 + if character % 16 < 10 {0x30} else {0x37};
    let bigger = character / 16 + if character / 16 < 10 {0x30} else {0x37};
    (bigger, smaller)
}