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
    Char(char),
    Meta,
    Enter,
    Right,
    Shift,
    Space,
    Tab,
    Up,
    Code(u16),
    Super,
}

pub trait Keyboard {
    fn send_chord(&self, keys: &[Key], process: &process::Child) -> Option<()>;
}
