use std::io::{Write, Result};

use super::{Color, Formatting};
use crossterm::{style::Print, Command, queue};

pub struct PromptStyle {
    prefix: String,
    prefix_formatting: Formatting,
    prompt_formatting: Formatting,
}

impl PromptStyle {
    pub fn print<W, S>(&self, buffer: &mut W, text: S) -> Result<()>
    where
        W: Write,
        S: Into<String>,
    {
        queue!(
            buffer,
            self,
            Print(text.into()),
            Print(" "),
            Formatting::reset()
        )
    }
}

impl Default for PromptStyle {
    fn default() -> Self {
        PromptStyle {
            prefix: "?".into(),
            prefix_formatting: Formatting::default().bold().foreground_color(Color::Green),
            prompt_formatting: Formatting::default().bold(),
        }
    }
}

impl Command for PromptStyle {
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
