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

pub fn input<F>(label: &str, validation: Option<F>) -> InputPromt {
    InputPromt::new(label)
}

pub struct InputPromt {
    label: String,
    default_value: Option<String>,
    esc_interrupts: bool,
}

impl InputPromt {
    pub fn show<W>(mut self, buffer: &mut W) -> Result<String, Error>
    where
        W: Write,
    {
        self.show_validated(buffer, |s| Ok(()))
    }

    pub fn show_validated<W, F>(mut self, buffer: &mut W, validation: F) -> Result<String, Error>
    where
        W: Write,
        F: Fn(&str) -> Result<(), String>,
    {
        crate::draw_promt(buffer, &self.label, &self.default_value)?;

        let raw_mode = RawMode::ensure();

        let mut input = String::new();
        let mut result: Result<String, String> = self
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
                        // result = self
                        //     .validation
                        //     .map_or_else(|| Ok(input), |v| v(&input).and_then(|_| Ok(input)));
                    }
                    KeyCode::Backspace => (),
                    KeyCode::Enter => (),
                    KeyCode::Esc if self.esc_interrupts => {
                        result = Err(String::new());
                        break;
                    }
                    _ => (),
                },
                _ => (),
            }
        }

        raw_mode.drop();
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
