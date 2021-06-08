mod keyboard;
mod window;

use std::process;

use thiserror::Error;

use winapi::um::winuser::KEYEVENTF_KEYUP;

use crate::{Key, Keyboard};

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Window {
        #[from]
        source: window::Error,
    },
    #[error(transparent)]
    Keyboard {
        #[from]
        source: keyboard::Error,
    },
}

#[derive(Debug, Default)]
pub struct Innerput;

impl Innerput {
    pub fn new() -> Self {
        Innerput {}
    }
}

impl Keyboard<Error> for Innerput {
    /// Sends a key chord to the child process. All the keys are pressed before
    /// being released.
    fn send_chord(&self, keys: &[Key], process: &process::Child) -> Result<(), Error> {
        window::activate_top_level_window(process)?;

        let press = &mut keyboard::make_input(keys, 0)?;
        let release = &mut keyboard::make_input(keys, KEYEVENTF_KEYUP)?;

        press.append(release);
        keyboard::send_input(press);

        Ok(())
    }
}
