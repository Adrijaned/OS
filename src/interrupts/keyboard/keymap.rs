use interrupts::keyboard::keymap::Key::{CharKey, ModifierKey};
use interrupts::keyboard::keymap::ModifierKeyValues::{Escape, Enter, LCtrl, LShift, RShift, RAlt, LAlt};

/// Returns a `char` corresponding to given scancode and modifier level.
///
/// # Safety
/// `scancode` must be lesser than `0x80`;
pub unsafe fn get_char(scancode: u8, is_e0: bool, modifier_level: u8) -> & 'static Result<Key, ()> {
    let scancode = scancode as usize;
    match modifier_level {
        1 => if !is_e0 {
            ::io::status_bar::beep(2048);
            &LEVEL1[scancode]
        } else {
            &Err(())
        },
        _ => &Err(()),
    }
}

pub enum ModifierKeyValues {
    LShift,
    RShift,
    LCtrl,
    RCtrl,
    Escape,
    Enter,
    LAlt,
    RAlt
}

pub enum Key { CharKey(char), ModifierKey(ModifierKeyValues) }

static LEVEL1: [Result<Key, ()>; 0x80] = [
    Err(()), Ok(ModifierKey(Escape)), Ok(CharKey('1')), Ok(CharKey('2')), Ok(CharKey('3')), Ok(CharKey('4')), Ok(CharKey('5')), Ok(CharKey('6')),    // 00..07
    Ok(CharKey('7')), Ok(CharKey('8')), Ok(CharKey('9')), Ok(CharKey('0')), Ok(CharKey('-')), Ok(CharKey('=')), Ok(CharKey('\u{0008}')), Ok(CharKey('\u{0009}')),    // 08..0f
    Ok(CharKey('q')), Ok(CharKey('w')), Ok(CharKey('e')), Ok(CharKey('r')), Ok(CharKey('t')), Ok(CharKey('y')), Ok(CharKey('u')), Ok(CharKey('i')), // 10..17
    Ok(CharKey('o')), Ok(CharKey('p')), Ok(CharKey('[')), Ok(CharKey(']')), Ok(ModifierKey(Enter)), Ok(ModifierKey(LCtrl)), Ok(CharKey('a')), Ok(CharKey('s')), // 18..1f
    Ok(CharKey('d')), Ok(CharKey('f')), Ok(CharKey('g')), Ok(CharKey('h')), Ok(CharKey('j')), Ok(CharKey('k')), Ok(CharKey('l')), Ok(CharKey(';')), // 20..27
    Ok(CharKey('\'')), Ok(CharKey('`')), Ok(ModifierKey(LShift)), Ok(CharKey('\\')), Ok(CharKey('z')), Ok(CharKey('x')), Ok(CharKey('c')), Ok(CharKey('v')), // 28..2f
    Ok(CharKey('b')), Ok(CharKey('n')), Ok(CharKey('m')), Ok(CharKey(',')), Ok(CharKey('.')), Ok(CharKey('/')), Ok(ModifierKey(RShift)), Ok(CharKey('*')), // 30..37
    Ok(ModifierKey(LAlt)), Ok(CharKey(' ')), Err(()), Err(()), Err(()), Err(()), Err(()), Err(()), // 38..3f
    Err(()), Err(()), Err(()), Err(()), Err(()), Err(()), Err(()), Err(()), // 40..47
    Err(()), Err(()), Err(()), Err(()), Err(()), Err(()), Err(()), Err(()), // 48..4f
    Err(()), Err(()), Err(()), Err(()), Err(()), Err(()), Err(()), Err(()), // 50..57
    Err(()), Err(()), Err(()), Err(()), Err(()), Err(()), Err(()), Err(()), // 58..5f
    Err(()), Err(()), Err(()), Err(()), Err(()), Err(()), Err(()), Err(()), // 60..67
    Err(()), Err(()), Err(()), Err(()), Err(()), Err(()), Err(()), Err(()), // 68..6f
    Err(()), Err(()), Err(()), Err(()), Err(()), Err(()), Err(()), Err(()), // 70..77
    Err(()), Err(()), Err(()), Err(()), Err(()), Err(()), Err(()), Err(())  // 78..7f
];