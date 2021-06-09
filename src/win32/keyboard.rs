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
    match key {
        Key::Alt => Ok(VK_MENU),
        Key::Backspace => Ok(VK_BACK),
        Key::CapsLock => Ok(VK_CAPITAL),
        Key::Char(character) => keycode_from_char(character.to_string()),
        Key::Control => Ok(VK_CONTROL),
        Key::Delete => Ok(VK_DELETE),
        Key::Down => Ok(VK_DOWN),
        Key::End => Ok(VK_END),
        Key::Esc => Ok(VK_ESCAPE),
        Key::Home => Ok(VK_HOME),
        Key::Left => Ok(VK_LEFT),
        Key::Enter => Ok(VK_RETURN),
        Key::Right => Ok(VK_RIGHT),
        Key::Shift => Ok(VK_SHIFT),
        Key::Space => Ok(VK_SPACE),
        Key::Super => Ok(VK_LWIN),
        Key::Tab => Ok(VK_TAB),
        Key::Up => Ok(VK_UP),
    }
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
        assert_eq!(get_keycode(&Key::Char('s')).unwrap(), 0x53);
        assert_eq!(get_keycode(&Key::Char('1')).unwrap(), 0x31);
    }
}
