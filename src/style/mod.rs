mod formatting;
mod color;
mod label_style;
mod prompts;

pub use color::Color;
pub use formatting::{Formatting, FormattingOption};
pub use label_style::LabelStyle;
pub use prompts::{input::InputStyle, confirmation::ConfirmationStyle};
