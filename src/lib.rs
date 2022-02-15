mod error;
mod promts;

pub use promts::confirmation::confirmation;
pub use promts::input::input;

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
) -> Result<(), error::Error>
where
    W: std::io::Write,
{
    queue!(
        buffer,
        SetAttribute(Attribute::Bold),
        SetForegroundColor(Color::Green),
        Print("? ".to_string()),
        ResetColor
    )
    .unwrap_or_default();

    queue!(
        buffer,
        SetAttribute(Attribute::Bold),
        Print(format!("{}: ", label)),
        SetAttribute(Attribute::Reset)
    )
    .unwrap_or_default();

    queue!(buffer, SavePosition).unwrap_or_default();

    if let Some(default) = default_value {
        queue!(
            buffer,
            SetForegroundColor(Color::DarkGrey),
            Print(format!("[{}]", default)),
            ResetColor
        )
        .unwrap_or_default();
    }

    buffer.flush().map_err(|err| err.into())
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
