use crossterm::event::{Event, KeyCode};

use super::Prompt;
use crate::{
    engine::CommandBuffer,
    prompts::{AbortReason, EventOutcome},
    style::{Formatting, InputStyle, LabelStyle},
};

pub struct Input<F> {
    label: String,
    input: String,
    help_message: Option<String>,
    is_first_input: bool,
    is_submitted: bool,
    error: Option<String>,
    validation: F,
    style: InputStyle,
}

impl<F, T> Input<F>
where
    F: Fn(&str) -> Result<T, String>,
{
    pub fn new(label: impl Into<String>, validation: F) -> Self {
        Self {
            label: label.into(),
            input: String::new(),
            help_message: None,
            is_first_input: true,
            is_submitted: false,
            error: None,
            validation,
            style: InputStyle::default(),
        }
    }

    pub fn help_message<S: Into<String>>(mut self, message: S) -> Self {
        self.help_message = Some(message.into());
        self
    }

    pub fn default_value<S: Into<String>>(mut self, val: S) -> Self {
        self.input = val.into();
        self
    }

    pub fn style(mut self, style: InputStyle) -> Self {
        self.style = style;
        self
    }
}

impl<T, F> Prompt<T> for Input<F>
where
    F: Fn(&str) -> Result<T, String>,
{
    fn draw(&self, commands: &mut impl CommandBuffer) {
        self.style.label_style.print_cmd(&self.label, commands);

        if let Some(error) = self.error.as_ref() {
            self.style.error_formatting.print_cmd(format!("[{}]", error), commands);
        } else if self.is_submitted {
            self.style.submitted_formatting.print_cmd(&self.input, commands);
        } else if self.is_first_input && self.input.len() > 0 {
            self.style.default_value_formatting.print_cmd(format!("[{}]", self.input), commands);
        } else if !self.is_first_input {
            self.style.input_formatting.print_cmd(&self.input, commands);
        }

        if let Some(help_message) = self.help_message.as_ref() {
            self.style.help_message_formatting.print_cmd(format!("[{}]", help_message), commands);
        }
    }

    fn on_event(&mut self, evt: Event) -> EventOutcome<T> {
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
