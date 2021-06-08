use std::mem::size_of;

use winapi::{
    ctypes::c_int,
    shared::minwindef::{DWORD, WORD},
    um::winuser::{INPUT_u, SendInput, VkKeyScanA, INPUT, INPUT_KEYBOARD, KEYBDINPUT},
};

use crate::Key;

// https://msdn.microsoft.com/en-us/library/windows/desktop/dd375731
const VK_RETURN: u16 = 0x0D;
const VK_TAB: u16 = 0x09;
const VK_SPACE: u16 = 0x20;
const VK_BACK: u16 = 0x08;
const VK_ESCAPE: u16 = 0x1b;
const VK_LWIN: u16 = 0x5b;
const VK_SHIFT: u16 = 0x10;
const VK_CAPITAL: u16 = 0x14;
const VK_MENU: u16 = 0x12;
const VK_LCONTROL: u16 = 0x11;
const VK_HOME: u16 = 0x24;
const VK_END: u16 = 0x23;
const VK_LEFT: u16 = 0x25;
const VK_RIGHT: u16 = 0x27;
const VK_UP: u16 = 0x26;
const VK_DOWN: u16 = 0x28;
const VK_DELETE: u16 = 0x2E;

fn keycode_from_char(string: String) -> WORD {
    let buf = &mut [0u8; 2];
    let result = string
        .chars()
        .next()
        .expect("Invalid character passed as input")
        .encode_utf8(buf)
        .bytes()
        .next();

    if result.is_none() {
        panic!("Invalid character passed as input");
    }

    (unsafe { VkKeyScanA(result.unwrap() as i8) }) as WORD
}

fn get_keycode(key: &Key) -> WORD {
    match key {
        Key::Alt => VK_MENU,
        Key::Backspace => VK_BACK,
        Key::CapsLock => VK_CAPITAL,
        Key::Char(character) => keycode_from_char(character.to_string()),
        Key::Control => VK_LCONTROL,
        Key::Delete => VK_DELETE,
        Key::Down => VK_DOWN,
        Key::End => VK_END,
        Key::Esc => VK_ESCAPE,
        Key::Home => VK_HOME,
        Key::Left => VK_LEFT,
        Key::Enter => VK_RETURN,
        Key::Right => VK_RIGHT,
        Key::Shift => VK_SHIFT,
        Key::Space => VK_SPACE,
        Key::Super => VK_LWIN,
        Key::Tab => VK_TAB,
        Key::Up => VK_UP,
    }
}

pub(crate) fn make_input(keys: &[Key], flags: DWORD) -> Vec<INPUT> {
    keys.iter()
        .map(|key| {
            let mut input = INPUT_u::default();

            unsafe {
                *input.ki_mut() = KEYBDINPUT {
                    dwExtraInfo: 0,
                    dwFlags: flags,
                    time: 0,
                    wScan: 0,
                    wVk: get_keycode(key),
                };
            };

            INPUT {
                type_: INPUT_KEYBOARD,
                u: input,
            }
        })
        .collect()
}

pub(crate) fn send_input(input: &mut Vec<INPUT>) {
    unsafe {
        SendInput(
            input.len() as DWORD,
            input.as_mut_ptr(),
            size_of::<INPUT>() as c_int,
        )
    };
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn char_to_keycode() {
        assert_eq!(get_keycode(&Key::Char('s')), 0x53)
    }
}
