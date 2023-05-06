use std::io::Write;

use crossterm::{queue, style::{SetAttribute, SetForegroundColor, Print, ResetColor, Attribute, Color}};

pub fn draw_prompt<W: Write, S: Into<String>>(buffer: &mut W, label: S) {
    queue!(
        buffer,
        SetAttribute(Attribute::Bold),
        SetForegroundColor(Color::Green),
        Print("? ".to_string()),
        ResetColor
    )
    .unwrap();

    queue!(
        buffer,
        SetAttribute(Attribute::Bold),
        Print(format!("{}: ", label.into())),
        SetAttribute(Attribute::Reset)
    )
    .unwrap();
}
