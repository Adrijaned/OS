/// Returns a `char` corresponding to given scancode and modifier level.
///
/// # Safety
/// `scancode` must be lesser than `0x80`;
pub unsafe fn get_char(scancode: u8, is_e0: bool, modifier_level: u8) -> Result<Option<char>, ()> {
    let scancode = scancode as usize;
    match modifier_level {
        1 => if !is_e0 {
            ::io::status_bar::beep(2048);
            LEVEL1[scancode]
        } else {
            Err(())
        },
        _ => Err(()),
    }
}

static LEVEL1: [Result<Option<char>, ()>; 0x80] = [
    Err(()), Ok(None), Ok(Some('1')), Ok(Some('2')), Ok(Some('3')), Ok(Some('4')), Ok(Some('5')), Ok(Some('6')),    // 00..07
    Ok(Some('7')), Ok(Some('8')), Ok(Some('9')), Ok(Some('0')), Ok(Some('-')), Ok(Some('=')), Ok(Some('\u{0008}')), Ok(Some('\u{0009}')),    // 08..0f
    Ok(Some('q')), Ok(Some('w')), Ok(Some('e')), Ok(Some('r')), Ok(Some('t')), Ok(Some('y')), Ok(Some('u')), Ok(Some('i')), // 10..17
    Ok(Some('o')), Ok(Some('p')), Ok(Some('[')), Ok(Some(']')), Err(()), Err(()), Ok(Some('a')), Ok(Some('s')), // 18..1f
    Err(()), Err(()), Err(()), Err(()), Err(()), Err(()), Err(()), Err(()), // 20..27
    Err(()), Err(()), Err(()), Err(()), Err(()), Err(()), Err(()), Err(()), // 28..2f
    Err(()), Err(()), Err(()), Err(()), Err(()), Err(()), Err(()), Err(()), // 30..37
    Err(()), Err(()), Err(()), Err(()), Err(()), Err(()), Err(()), Err(()), // 38..3f
    Err(()), Err(()), Err(()), Err(()), Err(()), Err(()), Err(()), Err(()), // 40..47
    Err(()), Err(()), Err(()), Err(()), Err(()), Err(()), Err(()), Err(()), // 48..4f
    Err(()), Err(()), Err(()), Err(()), Err(()), Err(()), Err(()), Err(()), // 50..57
    Err(()), Err(()), Err(()), Err(()), Err(()), Err(()), Err(()), Err(()), // 58..5f
    Err(()), Err(()), Err(()), Err(()), Err(()), Err(()), Err(()), Err(()), // 60..67
    Err(()), Err(()), Err(()), Err(()), Err(()), Err(()), Err(()), Err(()), // 68..6f
    Err(()), Err(()), Err(()), Err(()), Err(()), Err(()), Err(()), Err(()), // 70..77
    Err(()), Err(()), Err(()), Err(()), Err(()), Err(()), Err(()), Err(())  // 78..7f
];