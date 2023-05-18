
pub enum Key {
    Backspace,
    Enter,
    Left,
    Right,
    Up,
    Down,

    Home,
    End,
    PageUp,
    PageDown,

    Tab,
    BackTab,
    Delete,
    Insert,

    F(u8),
    Char(char),
    Esc,
}
