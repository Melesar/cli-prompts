use std::io::{Result, Write};

use super::color::Color;
use crossterm::{
    queue,
    style::{Attribute, Attributes, Colors, Print, SetAttributes, SetColors},
    Command,
};

#[derive(Clone, Copy)]
pub enum FormattingOption {
    Reset,
    Bold,
    Italic,
    Underline,
    CrossedOut,
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

pub struct Formatting {
    foreground_color: Option<Color>,
    background_color: Option<Color>,
    text_formatting: Vec<FormattingOption>,
}

impl Default for Formatting {
    fn default() -> Self {
        Formatting {
            foreground_color: None,
            background_color: None,
            text_formatting: vec![],
        }
    }
}

impl Formatting {
    pub fn foreground_color(mut self, color: Color) -> Self {
        self.foreground_color = Some(color);
        self
    }

    pub fn background_color(mut self, color: Color) -> Self {
        self.background_color = Some(color);
        self
    }

    pub fn bold(mut self) -> Self {
        self.text_formatting.push(FormattingOption::Bold);
        self
    }

    pub fn italic(mut self) -> Self {
        self.text_formatting.push(FormattingOption::Italic);
        self
    }

    pub fn underline(mut self) -> Self {
        self.text_formatting.push(FormattingOption::Underline);
        self
    }

    pub fn crossed_out(mut self) -> Self {
        self.text_formatting.push(FormattingOption::CrossedOut);
        self
    }

    pub fn reset() -> Self {
        let mut f = Self::default();
        f.text_formatting.push(FormattingOption::Reset);
        f
    }

    pub fn print<W, S>(&self, buffer: &mut W, text: S) -> Result<()>
    where
        W: Write,
        S: Into<String>,
    {
        queue!(buffer, self, Print(text.into()), Self::reset())
    }
}

impl Command for Formatting {
    fn write_ansi(&self, f: &mut impl std::fmt::Write) -> std::fmt::Result {
        self.get_colors().write_ansi(f)?;
        self.get_attributes().write_ansi(f)?;

        Ok(())
    }

    #[cfg(windows)]
    fn execute_winapi(&self) -> Result<()> {
        self.get_colors().execute_winapi()?;
        self.get_attributes().execute_winapi()?;

        Ok(())
    }
}

impl Formatting {
    fn get_colors(&self) -> SetColors {
        SetColors(Colors {
            foreground: self.foreground_color.map(|c| c.into()),
            background: self.background_color.map(|c| c.into()),
        })
    }

    fn get_attributes(&self) -> SetAttributes {
        let attributes_vec: Vec<Attribute> =
            self.text_formatting.iter().map(|&f| f.into()).collect();
        let attributes_ref: &[Attribute] = &attributes_vec;
        let attributes: Attributes = attributes_ref.into();
        SetAttributes(attributes)
    }
}
