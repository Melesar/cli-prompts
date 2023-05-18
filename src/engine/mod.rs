mod crossterm;

pub use self::crossterm::CrosstermEngine;

use std::io::Result;
use crate::{style::Formatting, input::Key};

pub trait Engine {
    type Buffer: CommandBuffer + Clear;

    fn get_command_buffer(&self) -> Self::Buffer;

    fn render(&mut self, render_commands: &Self::Buffer) -> Result<()>;
    fn finish_rendering(&mut self) -> Result<()>;

    fn read_key(&self) -> Result<Key>;
}

pub trait Clear {
    fn clear(&mut self);
}

pub trait CommandBuffer {
    fn new_line(&mut self);

    fn print(&mut self, text: &str);

    fn set_formatting(&mut self, formatting: &Formatting);

    fn reset_formatting(&mut self);
}
