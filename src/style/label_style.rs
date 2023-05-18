use crate::engine::CommandBuffer;

use super::{Color, Formatting};

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

    pub fn print(&self, text: impl Into<String>, cmd_buffer: &mut impl CommandBuffer) {
        cmd_buffer.set_formatting(&self.prefix_formatting);
        cmd_buffer.print(&self.prefix);
        cmd_buffer.reset_formatting();
        cmd_buffer.print(" ");
        cmd_buffer.set_formatting(&self.prompt_formatting);
        cmd_buffer.print(&text.into());
        cmd_buffer.print(":");
        cmd_buffer.reset_formatting();
        cmd_buffer.print(" ");
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
