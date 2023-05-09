use std::io::Write;

use crossterm::{queue, style::{SetAttribute, SetForegroundColor, Print, ResetColor, Attribute, Color}};

pub fn draw_prompt<W: Write, S: Into<String>>(buffer: &mut W, label: S) -> Result<(), std::io::Error>{
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
        Print(format!("{}: ", label.into())),
        SetAttribute(Attribute::Reset)
    )?;

    Ok(())
}
