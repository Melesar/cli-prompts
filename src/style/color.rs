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

