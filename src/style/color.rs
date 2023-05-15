
use crossterm::style::Color as Cc;

#[derive(Copy, Clone)]
pub enum Color {
    Reset,

    Black,
    DarkGrey,

    Red,
    DarkRed,

    Green,
    DarkGreen,

    Yellow,
    DarkYellow,
    Blue,
    DarkBlue,

    Magenta,
    DarkMagenta,

    Cyan,
    DarkCyan,

    White,
    Grey,

    Rgb { r: u8, g: u8, b: u8 },
    AnsiValue(u8),
}

impl From<Color> for Cc {
    fn from(value: Color) -> Self {
        match value {
            Color::Reset => Cc::Reset,
            Color::Black => Cc::Black,
            Color::DarkGrey => Cc::DarkGrey,
            Color::Red => Cc::Red,
            Color::DarkRed => Cc::DarkRed,
            Color::Green => Cc::Green,
            Color::DarkGreen => Cc::DarkGreen,
            Color::Yellow => Cc::Yellow,
            Color::DarkYellow => Cc::DarkYellow,
            Color::Blue => Cc::Blue,
            Color::DarkBlue => Cc::DarkBlue,
            Color::Magenta => Cc::Magenta,
            Color::DarkMagenta => Cc::DarkMagenta,
            Color::Cyan => Cc::Cyan,
            Color::DarkCyan => Cc::DarkCyan,
            Color::White => Cc::White,
            Color::Grey => Cc::Grey,
            Color::Rgb { r, g, b } => Cc::Rgb {r, g, b},
            Color::AnsiValue(c)=> Cc::AnsiValue(c),
        }
    }
}
