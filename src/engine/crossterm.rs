use std::io::{Write, Result};

use crossterm::{
    cursor::{position, MoveTo, MoveToPreviousLine},
    queue,
    terminal::{Clear, ClearType}, style::{Print, SetColors, Colors, Attribute, Attributes, SetAttributes}, execute,
};


use crate::style::{Formatting, FormattingOption};

use super::{CommandBuffer, Engine};

pub struct CrosstermEngine<W: Write> {
    buffer: W,
    previous_line_count: u16,
}

pub struct CrosstermCommandBuffer<W: Write> {
    commands: Vec<Box<dyn Command<W>>>,
    lines_count: u16,
}

impl<W: Write> CrosstermEngine<W> {
    pub fn new(buffer: W) -> Self {
        CrosstermEngine {
            buffer,
            previous_line_count: 1,
        }
    }
}

impl<W: Write> Engine for CrosstermEngine<W> {
    type Buffer = CrosstermCommandBuffer<W>;

    fn get_command_buffer(&self) -> Self::Buffer {
        CrosstermCommandBuffer::new()
    }

    fn render(&mut self, render_commands: &Self::Buffer) -> Result<()> {
        for i in 0..self.previous_line_count {
            queue!(self.buffer, Clear(ClearType::CurrentLine))?;
            if i < self.previous_line_count - 1 {
                queue!(self.buffer, MoveToPreviousLine(1))?;
            }
        }

        queue!(self.buffer, MoveTo(0, position()?.1))?;

        for cmd in &render_commands.commands {
            cmd.execute(&mut self.buffer)?;
        }

        self.previous_line_count = render_commands.lines_count;
        self.buffer.flush()
    }

    fn finish_rendering(&mut self) -> Result<()> {
        execute!(self.buffer, Print("\r\n"))
    }
}

impl<W: Write> CrosstermCommandBuffer<W> {
    fn new() -> Self {
        CrosstermCommandBuffer {
            commands: vec![],
            lines_count: 1,
        }
    }
}

impl<W: Write> CommandBuffer for CrosstermCommandBuffer<W> {
    fn new_line(&mut self) {
        self.commands.push(Box::new(NewLineCommand));
        self.lines_count += 1;
    }

    fn print(&mut self, text: &str) {
        self.commands.push(Box::new(PrintCommand(text.to_owned())));
    }

    fn set_formatting(&mut self, formatting: &Formatting) {
        self.commands
            .push(Box::new(SetFormattingCommand(formatting.to_owned())));
    }

    fn reset_formatting(&mut self) {
        self.commands
            .push(Box::new(SetFormattingCommand(Formatting::reset())));
    }
}

impl<W: Write> super::Clear for CrosstermCommandBuffer<W> {
    fn clear(&mut self) {
        self.commands.clear();
        self.lines_count = 1;
    }
}

struct NewLineCommand;
struct PrintCommand(String);
struct SetFormattingCommand(Formatting);

trait Command<W: Write> {
    fn execute(&self, buffer: &mut W) -> Result<()>;
}

impl<W: Write> Command<W> for PrintCommand {
    fn execute(&self, buffer: &mut W) -> Result<()> {
        queue!(buffer, Print(&self.0))
    }
}

impl<W: Write> Command<W> for NewLineCommand {
    fn execute(&self, buffer: &mut W) -> Result<()> {
        queue!(buffer, Print("\r\n"))
    }
}

impl<W: Write> Command<W> for SetFormattingCommand {
    fn execute(&self, buffer: &mut W) -> Result<()> {
        let colors = Colors {
            foreground: self.0.foreground_color.map(|c| c.into()),
            background: self.0.background_color.map(|c| c.into()),
        };

        let attributes_vec: Vec<Attribute> =
            self.0.text_formatting.iter().map(|&f| f.into()).collect();
        let attributes_ref: &[Attribute] = &attributes_vec;
        let attributes: Attributes = attributes_ref.into();

        queue!(buffer, SetColors(colors), SetAttributes(attributes))
    }
}

impl From<FormattingOption> for crossterm::style::Attribute {
    fn from(value: FormattingOption) -> Self {
        match value {
            FormattingOption::Reset => Attribute::Reset,
            FormattingOption::Bold => Attribute::Bold,
            FormattingOption::Italic => Attribute::Italic,
            FormattingOption::Underline => Attribute::Underlined,
            FormattingOption::CrossedOut => Attribute::CrossedOut,
        }
    }
}
