use crate::error::Error;
use crate::RawMode;
use std::io::Write;

use crossterm::{
    cursor::RestorePosition,
    event::{read, Event, KeyCode},
    execute, queue,
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{Clear, ClearType},
};

pub fn input(label: &str) -> InputPromt {
    InputPromt::new(label)
}

pub struct InputPromt {
    label: String,
    default_value: Option<String>,
    esc_interrupts: bool,
}

impl InputPromt {
    pub fn show<W>(self, buffer: &mut W) -> Result<String, Error>
    where
        W: Write,
    {
        self.show_validated(buffer, |_| Ok(()))
    }

    pub fn show_validated<W, F>(self, buffer: &mut W, validation: F) -> Result<String, Error>
    where
        W: Write,
        F: Fn(&str) -> Result<(), String>,
    {
        crate::draw_promt(buffer, &self.label, &self.default_value)?;

        let raw_mode = RawMode::ensure();

        let mut input = String::new();
        let mut result = self
            .default_value
            .clone()
            .ok_or(String::new())
            .and_then(|r| {
                let validation = validation(&r);
                if validation.is_ok() {
                    Ok(r)
                } else {
                    Err(validation.unwrap_err())
                }
            });

        loop {
            match read().map_err(|e| Error::IoError(e))? {
                Event::Key(k) => match k.code {
                    KeyCode::Char(c) => {
                        input.push(c);
                        result = validation(&input).map(|_| input.clone());
                        execute!(
                            buffer,
                            RestorePosition,
                            Clear(ClearType::UntilNewLine),
                            Print(&input)
                        )
                        .unwrap_or_default();
                    }
                    KeyCode::Backspace => {
                        input.pop();
                        result = validation(&input).map(|_| input.clone());
                        execute!(
                            buffer,
                            RestorePosition,
                            Clear(ClearType::UntilNewLine),
                            Print(&input)
                        )
                        .unwrap_or_default();
                    }
                    KeyCode::Enter => match result.as_ref() {
                        Ok(_) => break,
                        Err(e) => {
                            input.clear();
                            display_error(buffer, e)
                        }
                    },
                    KeyCode::Esc if self.esc_interrupts => {
                        result = Err(String::new());
                        break;
                    }
                    _ => (),
                },
                _ => (),
            }
        }

        drop(raw_mode);

        if let Ok(r) = result.as_ref() {
            execute!(
                buffer,
                RestorePosition,
                Clear(ClearType::UntilNewLine),
                SetForegroundColor(Color::DarkCyan),
                Print(r),
                Print('\n'),
                ResetColor
            )
            .unwrap_or_default();
        }

        result.map_err(|_| Error::Interrupted)
    }

    pub fn default_value(mut self, val: String) -> Self {
        self.default_value = Some(val);
        self
    }

    pub fn esc_interrupts(mut self, interrupts: bool) -> Self {
        self.esc_interrupts = interrupts;
        self
    }

    fn new(label: &str) -> Self {
        InputPromt {
            label: label.into(),
            default_value: None,
            esc_interrupts: false,
        }
    }
}

fn display_error<W>(buffer: &mut W, error: &String)
where
    W: Write,
{
    queue!(buffer, RestorePosition, Clear(ClearType::UntilNewLine)).unwrap_or_default();
    queue!(
        buffer,
        Print(" "),
        SetForegroundColor(Color::Red),
        Print(format!("[{}]", error)),
        ResetColor
    )
    .unwrap_or_default();
    queue!(buffer, RestorePosition).unwrap_or_default();

    buffer.flush().unwrap_or_default();
}
