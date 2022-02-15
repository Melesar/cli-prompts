use std::io::Write;

use crate::draw_promt;

pub fn confirmation() -> Confirmation {
    Confirmation::default()
}

pub struct Confirmation {
    default_positive: bool,
}

impl Confirmation {
    pub fn show<W>(self, buffer: &mut W, label: &str) -> Result<bool, crate::error::Error>
    where
        W: Write,
    {
        draw_promt(
            buffer,
            &format!(
                "{} [{yes}/{no}]",
                label,
                yes = if self.default_positive { 'Y' } else { 'y' },
                no = if !self.default_positive { 'N' } else { 'n' }
            ),
            &None::<String>,
        )?;

        let stdin = std::io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap_or_default();

        let input = input.trim();
        if self.default_positive && input.eq("n") {
            Ok(false)
        } else if !self.default_positive && !input.eq("y") {
            Ok(false)
        } else {
            Ok(true)
        }
    }

    pub fn default_positive(mut self, default_positive: bool) -> Self {
        self.default_positive = default_positive;
        self
    }
}

impl Default for Confirmation {
    fn default() -> Self {
        Confirmation {
            default_positive: true,
        }
    }
}
