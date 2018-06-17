pub fn u8toascii(character: u8) -> (u8, u8, u8) {
    ((character / 100) + 0x30, ((character / 10) % 10) + 0x30, (character % 10) + 0x30)
}