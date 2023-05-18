use crate::engine::CommandBuffer;

use super::color::Color;

#[derive(Clone, Copy)]
pub enum FormattingOption {
    Reset,
    Bold,
    Italic,
    Underline,
    CrossedOut,
}

#[derive(Clone)]
pub struct Formatting {
    pub foreground_color: Option<Color>,
    pub background_color: Option<Color>,
    pub text_formatting: Vec<FormattingOption>,
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
    pub fn print(&self, text: impl Into<String>, cmd_buffer: &mut impl CommandBuffer) {
        cmd_buffer.set_formatting(self);
        cmd_buffer.print(&text.into());
        cmd_buffer.reset_formatting();
    }
}
