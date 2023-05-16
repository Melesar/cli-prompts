use std::io::{Write, Result};

use super::{Color, Formatting};
use crossterm::{style::{Print, SetAttribute, Attribute}, Command, queue};

#[derive(Clone)]
pub struct LabelStyle {
    prefix: String,
    prefix_formatting: Formatting,
    prompt_formatting: Formatting,
}

impl LabelStyle {
    pub fn prefix<S: Into<String>>(mut self, p: S) -> Self {
        self.prefix = p.into();
        self
    }

    pub fn prefix_formatting(mut self, f: Formatting) -> Self {
        self.prefix_formatting = f;
        self
    }

    pub fn prompt_formatting(mut self, f: Formatting) -> Self {
        self.prompt_formatting = f;
        self
    }

    pub fn print<W, S>(&self, buffer: &mut W, text: S) -> Result<()>
    where
        W: Write,
        S: Into<String>,
    {
        queue!(
            buffer,
            self,
            Print(text.into()),
            Print(":"),
            Formatting::reset(),
            Print(" "),
        )
    }
}

impl Default for LabelStyle {
    fn default() -> Self {
        LabelStyle {
            prefix: "?".into(),
            prefix_formatting: Formatting::default().bold().foreground_color(Color::Green),
            prompt_formatting: Formatting::default().bold(),
        }
    }
}

impl Command for LabelStyle {
    fn write_ansi(&self, f: &mut impl std::fmt::Write) -> std::fmt::Result {
        self.prefix_formatting.write_ansi(f)?;
        Print(&self.prefix).write_ansi(f)?;
        Formatting::reset().write_ansi(f)?;
        Print(" ").write_ansi(f)?;
        self.prompt_formatting.write_ansi(f)?;

        Ok(())
    }

    #[cfg(windows)]
    fn execute_winapi(&self) -> Result<()> {
        self.prefix_formatting.execute_winapi()?;
        Print(&self.prefix).execute_winapi()?;
        Formatting::reset().execute_winapi()?;
        Print(" ").execute_winapi()?;
        self.prompt_formatting.execute_winapi()?;

        Ok(())
    }
}
