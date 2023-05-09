use super::{EventOutcome, Prompt};
use crate::output::draw_prompt;

use crossterm::{
    queue,
    event::{Event, KeyCode}, style::{SetForegroundColor, Color, ResetColor, Print}, terminal::{Clear, ClearType}, cursor::{MoveTo, position}
};
use std::{io::Write, result};

pub struct Confirmation {
    label: String,
    default_positive: bool,
    is_submitted: bool,
    selected_option: Option<bool>,
}

impl Confirmation {
    pub fn new<S: Into<String>>(label: S) -> Self {
        Confirmation {
            label: label.into(),
            default_positive: true,
            is_submitted: false,
            selected_option: None,
        }
    }

    pub fn default_positive(mut self, default_positive: bool) -> Self {
        self.default_positive = default_positive;
        self
    }
}

impl Prompt<bool> for Confirmation {
    fn draw<W: Write>(&self, buffer: &mut W) -> Result<(), std::io::Error> {
        queue!(
            buffer,
            Clear(ClearType::CurrentLine),
            MoveTo(0, position()?.1)
        )?;
        
        draw_prompt(
            buffer,
            format!(
                "{} [{y}/{n}]",
                self.label,
                y = if self.default_positive { 'Y' } else { 'y' },
                n = if !self.default_positive { 'N' } else { 'n' },
            ),
        )?;

        let result = if let Some(is_positive) = self.selected_option.as_ref() {
            if *is_positive { "Yes" } else { "No" }
        } else { 
            ""
        };

        if self.is_submitted {
            queue!(buffer, SetForegroundColor(Color::Green))?;
        }

        queue!(buffer, Print(result), ResetColor)?;

        if self.is_submitted {
            queue!(buffer, Print("\r\n"))?;
        }

        buffer.flush()?;
        Ok(())
    }

    fn on_event(&mut self, evt: Event) -> EventOutcome<bool> {
        match evt {
            Event::Key(key) => match key.code {
                KeyCode::Enter => {
                    self.is_submitted = true;
                    if let Some(is_positive) = self.selected_option.as_ref() {
                        EventOutcome::Done(*is_positive)
                    } else {
                        self.selected_option = Some(self.default_positive);
                        EventOutcome::Done(self.default_positive)
                    }
                },
                KeyCode::Char(c) if self.selected_option.is_none() => match c {
                    'y' | 'Y' => {
                        self.selected_option = Some(true);
                        EventOutcome::Continue
                    }
                    'n' | 'N' => {
                        self.selected_option = Some(false);
                        EventOutcome::Continue
                    },
                    _ => EventOutcome::Continue
                },
                KeyCode::Backspace => {
                    self.selected_option = None;
                    EventOutcome::Continue
                },
                _ => EventOutcome::Continue,
            },
            _ => EventOutcome::Continue,
        }
    }
}
