#![allow(dead_code)]

mod promts;
mod output;
pub mod error;

pub use promts::confirmation::confirmation;
pub use promts::input::input;
pub use promts::selection::select_one;

pub use promts::refresh::input::Input;
pub use promts::refresh::confirmation::Confirmation;
pub use promts::refresh::selection::Selection;
pub use promts::refresh::display_prompt;

use error::Result;

use crossterm::{
    cursor::SavePosition,
    queue,
    style::{Attribute, Color, Print, ResetColor, SetAttribute, SetForegroundColor},
    terminal::{disable_raw_mode, enable_raw_mode, is_raw_mode_enabled},
};

pub(crate) fn draw_promt<W>(
    buffer: &mut W,
    label: &str,
    default_value: &Option<String>,
) -> Result<()>
where
    W: std::io::Write,
{
    queue!(
        buffer,
        SetAttribute(Attribute::Bold),
        SetForegroundColor(Color::Green),
        Print("? ".to_string()),
        ResetColor
    )?;

    queue!(
        buffer,
        SetAttribute(Attribute::Bold),
        Print(format!("{}: ", label)),
        SetAttribute(Attribute::Reset)
    )?;

    queue!(buffer, SavePosition)?;

    if let Some(default) = default_value {
        queue!(
            buffer,
            SetForegroundColor(Color::DarkGrey),
            Print(format!("[{}]", default)),
            ResetColor
        )?;
    }

    buffer.flush().map_err(|e| e.into())
}

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
