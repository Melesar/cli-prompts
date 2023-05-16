mod color;
mod formatting;
mod label_style;
mod option_marker;
mod prompts;

pub use color::Color;
pub use formatting::{Formatting, FormattingOption};
pub use label_style::LabelStyle;
pub use option_marker::OptionMarkerStyle;
pub use prompts::{
    confirmation::ConfirmationStyle,
    input::InputStyle,
    multiselection::MultiselectionStyle,
    selection::SelectionStyle,
};
