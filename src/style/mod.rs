mod formatting;
mod color;
mod prompt_style;

pub use color::Color;
pub use formatting::{Formatting, FormattingOption};
pub use prompt_style::PromptStyle;

pub trait StyledPrompt {
    type S;

    fn set_style(&mut self, style: Self::S);
}
