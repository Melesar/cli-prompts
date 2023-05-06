use crossterm::{
    cursor::{position, MoveTo},
    event::{Event, KeyCode},
    queue,
    style::{Attribute, Color, Print, ResetColor, SetAttribute, SetForegroundColor},
    terminal::{Clear, ClearType},
};
use std::io::Write;

use super::{AbortReason, EventOutcome, Prompt};
use crate::output::draw_prompt;

pub struct Input<F> {
    label: String,
    input: String,
    is_first_input: bool,
    is_submitted: bool,
    error: Option<String>,
    validation: F,
}

impl<F, T> Input<F>
where
    F: Fn(&str) -> Result<T, String>,
{
    pub fn new(label: impl Into<String>, validation: F) -> Self {
        Self {
            label: label.into(),
            input: String::new(),
            is_first_input: true,
            is_submitted: false,
            error: None,
            validation,
        }
    }

    pub fn default_value(mut self, val: String) -> Self {
        self.input = val;
        self
    }
}

impl<TOut, F> Prompt<TOut> for Input<F>
where
    F: Fn(&str) -> Result<TOut, String>,
{
    fn draw<W: Write>(&self, buffer: &mut W) {
        queue!(
            buffer,
            Clear(ClearType::CurrentLine),
            MoveTo(0, position().unwrap().1)
        )
        .unwrap();

        draw_prompt(buffer, &self.label);

        if self.error.is_some() {
            queue!(
                buffer,
                Print(" "),
                SetForegroundColor(Color::Red),
                Print(format!("[{}]", self.error.as_ref().unwrap())),
                ResetColor
            )
            .unwrap();
        } else if self.is_submitted {
            queue!(
                buffer,
                SetForegroundColor(Color::Green),
                Print(format!("{}\r\n", self.input)),
                ResetColor,
            )
            .unwrap();
        } else if self.is_first_input && self.input.len() > 0 {
            queue!(
                buffer,
                SetForegroundColor(Color::DarkGrey),
                Print(format!("[{}]", self.input)),
                ResetColor,
            )
            .unwrap();
        } else if !self.is_first_input {
            queue!(buffer, Print(format!("{}", self.input))).unwrap();
        }

        buffer.flush().unwrap_or_default();
    }

    fn on_event(&mut self, evt: Event) -> super::EventOutcome<TOut> {
        let is_first_input = self.is_first_input;
        match evt {
            Event::Key(k) => {
                self.is_first_input = false;
                match k.code {
                    KeyCode::Char(c) => {
                        if is_first_input {
                            self.input.clear();
                        }
                        self.error = None;
                        self.input.push(c);
                        EventOutcome::Continue
                    }
                    KeyCode::Backspace => {
                        if is_first_input {
                            self.input.clear();
                        }
                        self.error = None;
                        self.input.pop();
                        EventOutcome::Continue
                    }
                    KeyCode::Enter => {
                        self.error = (self.validation)(&self.input).err();
                        match self.error {
                            Some(_) => {
                                self.input.clear();
                                EventOutcome::Continue
                            }
                            None => {
                                self.is_submitted = true;
                                EventOutcome::Done((self.validation)(&self.input).unwrap())
                            }
                        }
                    }
                    KeyCode::Esc => EventOutcome::Abort(AbortReason::Interrupt),
                    _ => EventOutcome::Continue,
                }
            }
            _ => EventOutcome::Continue,
        }
    }
}
