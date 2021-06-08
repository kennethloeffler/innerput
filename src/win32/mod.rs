mod window;

use std::process;

use crate::{Key, Keyboard};

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
        unimplemented!()
    }
}
