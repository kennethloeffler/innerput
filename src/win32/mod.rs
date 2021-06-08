mod keyboard;
mod window;

use std::process;

use winapi::um::winuser::KEYEVENTF_KEYUP;

use crate::{Key, Keyboard};

#[derive(Default)]
pub struct Innerput;

impl Innerput {
    pub fn new() -> Self {
        Innerput {}
    }
}

impl Keyboard for Innerput {
    fn send_chord(&self, keys: &[Key], process: &process::Child) -> Option<()> {
        window::activate_top_level_window(process)?;

        let press = &mut keyboard::make_input(keys, 0);
        let release = &mut keyboard::make_input(keys, KEYEVENTF_KEYUP);

        press.append(release);
        keyboard::send_input(press);

        Some(())
    }
}
