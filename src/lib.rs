#![allow(dead_code)]

mod prompts;
mod output;

pub mod error;

pub use prompts::input::Input;
pub use prompts::confirmation::Confirmation;
pub use prompts::selection::Selection;
pub use prompts::display_prompt;

use crossterm::terminal::{disable_raw_mode, enable_raw_mode, is_raw_mode_enabled};

pub(crate) struct RawMode(bool);

impl RawMode {
    pub fn ensure() -> Self {
        let is_raw = is_raw_mode_enabled().unwrap_or(false);
        if !is_raw {
            enable_raw_mode().unwrap_or_default();
        }

        Self(is_raw)
    }
}

impl Drop for RawMode {
    fn drop(&mut self) {
        if !self.0 {
            disable_raw_mode().unwrap_or_default();
        }
    }
}
