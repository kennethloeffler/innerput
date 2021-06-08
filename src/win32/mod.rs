mod keyboard_input;
mod window;

use std::process;

use winapi::um::winuser::KEYEVENTF_KEYUP;

use crate::{Key, Keyboard};

use self::keyboard_input::{make_keybdinput, send_input};

pub struct Innerput;

impl Innerput {
    pub fn new() -> Self {
        Innerput {}
    }
}

impl Default for Innerput {
    fn default() -> Self {
        Self::new()
    }
}

impl Keyboard for Innerput {
    fn send_chord(&self, keys: &[Key], process: &process::Child) -> Option<()> {
        window::activate_top_level_window(process)?;

        let keys_down = &mut make_keybdinput(keys, 0);
        let keys_up = &mut make_keybdinput(keys, KEYEVENTF_KEYUP);

        keys_down.append(keys_up);
        send_input(keys_down);

        Some(())
    }
}
