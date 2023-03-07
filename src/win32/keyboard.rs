use std::mem::size_of;

use thiserror::Error;

use winapi::{
    ctypes::c_int,
    shared::minwindef::{DWORD, WORD},
    um::winuser::{INPUT_u, SendInput, VkKeyScanW, INPUT, INPUT_KEYBOARD, KEYBDINPUT},
};

use crate::Key;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Failed to convert character '{0}' to keycode")]
    CharConversionFailed(String),
}

// https://web.archive.org/web/20210404165500/https://docs.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes
const VK_RETURN: u16 = 0x0D;
const VK_TAB: u16 = 0x09;
const VK_SPACE: u16 = 0x20;
const VK_BACK: u16 = 0x08;
const VK_ESCAPE: u16 = 0x1b;
const VK_LWIN: u16 = 0x5b;
const VK_SHIFT: u16 = 0x10;
const VK_CAPITAL: u16 = 0x14;
const VK_MENU: u16 = 0x12;
const VK_CONTROL: u16 = 0x11;
const VK_HOME: u16 = 0x24;
const VK_END: u16 = 0x23;
const VK_LEFT: u16 = 0x25;
const VK_RIGHT: u16 = 0x27;
const VK_UP: u16 = 0x26;
const VK_DOWN: u16 = 0x28;
const VK_DELETE: u16 = 0x2E;
const VK_F1: u16 = 0x70;
const VK_F2: u16 = 0x71;
const VK_F3: u16 = 0x72;
const VK_F4: u16 = 0x73;
const VK_F5: u16 = 0x74;
const VK_F6: u16 = 0x75;
const VK_F7: u16 = 0x76;
const VK_F8: u16 = 0x77;
const VK_F9: u16 = 0x78;
const VK_F10: u16 = 0x79;
const VK_F11: u16 = 0x7A;
const VK_F12: u16 = 0x7B;

fn keycode_from_char(char_as_string: String) -> Result<WORD, Error> {
    let buf = &mut [0; 2];

    let character = char_as_string
        .chars()
        .next()
        .ok_or_else(|| Error::CharConversionFailed(char_as_string.to_string()))?
        .encode_utf16(buf);

    if character.len() != 1 {
        return Err(Error::CharConversionFailed(char_as_string));
    }

    // MSDN: The low order byte is the virtual keycode, while the high order
    // byte is the shift state.
    Ok((unsafe { VkKeyScanW(character[0]) }.to_le_bytes()[0]) as WORD)
}

fn get_keycode(key: &Key) -> Result<WORD, Error> {
    Ok(match key {
        Key::Alt => VK_MENU,
        Key::Backspace => VK_BACK,
        Key::CapsLock => VK_CAPITAL,
        Key::Char(character) => keycode_from_char(character.to_string())?,
        Key::Control => VK_CONTROL,
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
        Key::F1 => VK_F1,
        Key::F2 => VK_F2,
        Key::F3 => VK_F3,
        Key::F4 => VK_F4,
        Key::F5 => VK_F5,
        Key::F6 => VK_F6,
        Key::F7 => VK_F7,
        Key::F8 => VK_F8,
        Key::F9 => VK_F9,
        Key::F10 => VK_F10,
        Key::F11 => VK_F11,
        Key::F12 => VK_F12,
    })
}

pub(crate) fn make_input(keys: &[Key], flags: DWORD) -> Result<Vec<INPUT>, Error> {
    keys.iter()
        .map(|key| -> Result<INPUT, Error> {
            let mut input = INPUT_u::default();

            unsafe {
                *input.ki_mut() = KEYBDINPUT {
                    dwExtraInfo: 0,
                    dwFlags: flags,
                    time: 0,
                    wScan: 0,
                    wVk: get_keycode(key)?,
                };
            };

            Ok(INPUT {
                type_: INPUT_KEYBOARD,
                u: input,
            })
        })
        .collect()
}

pub(crate) fn send_input(input: &mut [INPUT]) {
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
        assert_eq!(get_keycode(&Key::Char('s')).unwrap(), 0x53);
        assert_eq!(get_keycode(&Key::Char('1')).unwrap(), 0x31);
    }
}
