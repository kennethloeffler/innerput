use std::process;

#[cfg(target_os = "windows")]
mod win32;
#[cfg(target_os = "windows")]
pub use win32::Innerput;

pub enum Key {
    Alt,
    Backspace,
    CapsLock,
    Control,
    Delete,
    Down,
    End,
    Esc,
    Home,
    Left,
    Enter,
    Right,
    Shift,
    Space,
    Tab,
    Up,
    Char(char),
    Super,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
}

mod private {
    pub trait Sealed {}

    impl Sealed for super::Innerput {}
}

pub trait Keyboard<ErrorType>: private::Sealed {
    fn send_chord(&self, keys: &[Key], process: &process::Child) -> Result<(), ErrorType>;
}
