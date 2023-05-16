use crossterm::{Command, style::Print};

use super::Formatting;

pub struct OptionMarkerStyle {
    pub marker: String,
    pub formatting: Formatting,
}

impl Command for OptionMarkerStyle {
    fn write_ansi(&self, f: &mut impl std::fmt::Write) -> std::fmt::Result {
        self.formatting.write_ansi(f)?;
        Print(&self.marker).write_ansi(f)?;
        Formatting::reset().write_ansi(f)?;

        Ok(())
    }

    #[cfg(windows)]
    fn execute_winapi(&self) -> Result<()> {
        self.formatting.execute_winapi()?;
        Print(&self.marker).execute_winapi()?;
        Formatting::reset().execute_winapi()?;

        Ok(())
    }
}



