pub mod input {
    use crate::style::{Color, Formatting, LabelStyle};

    pub struct InputStyle {
        pub label_style: LabelStyle,
        pub default_value_formatting: Formatting,
        pub error_formatting: Formatting,
        pub input_formatting: Formatting,
        pub submitted_formatting: Formatting,
        pub help_message_formatting: Formatting,
    }

    impl Default for InputStyle {
        fn default() -> Self {
            InputStyle {
                label_style: LabelStyle::default(),
                default_value_formatting: Formatting::default().foreground_color(Color::Grey),
                error_formatting: Formatting::default().foreground_color(Color::Red),
                input_formatting: Formatting::default(),
                submitted_formatting: Formatting::default().foreground_color(Color::Green),
                help_message_formatting: Formatting::default().foreground_color(Color::DarkGreen),
            }
        }
    }

    impl InputStyle {
        pub fn label_style(mut self, l: LabelStyle) -> Self {
            self.label_style = l;
            self
        }

        pub fn default_value_formatting(mut self, f: Formatting) -> Self {
            self.default_value_formatting = f;
            self
        }

        pub fn error_formatting(mut self, f: Formatting) -> Self {
            self.error_formatting = f;
            self
        }

        pub fn input_formatting(mut self, f: Formatting) -> Self {
            self.input_formatting = f;
            self
        }

        pub fn submitted_formatting(mut self, f: Formatting) -> Self {
            self.submitted_formatting = f;
            self
        }

        pub fn help_message_formatting(mut self, f: Formatting) -> Self {
            self.help_message_formatting = f;
            self
        }
    }
}

pub mod confirmation {
    use crate::style::{Color, Formatting, LabelStyle};

    pub struct ConfirmationStyle {
        pub label_style: LabelStyle,
        pub input_formatting: Formatting,
        pub submitted_formatting: Formatting,
    }

    impl Default for ConfirmationStyle {
        fn default() -> Self {
            ConfirmationStyle {
                label_style: LabelStyle::default(),
                input_formatting: Formatting::default(),
                submitted_formatting: Formatting::default().foreground_color(Color::Green),
            }
        }
    }

    impl ConfirmationStyle {
        pub fn label_style(mut self, l: LabelStyle) -> Self {
            self.label_style = l;
            self
        }

        pub fn input_formatting(mut self, f: Formatting) -> Self {
            self.input_formatting = f;
            self
        }

        pub fn submitted_formatting(mut self, f: Formatting) -> Self {
            self.submitted_formatting = f;
            self
        }
    }
}
