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

pub mod selection {
    use crate::{
        engine::CommandBuffer,
        style::{Color, Formatting, LabelStyle},
    };

    pub struct Marker {
        pub marker: String,
        pub formatting: Formatting,
    }

    pub struct SelectionStyle {
        pub label_style: LabelStyle,
        pub submitted_formatting: Formatting,
        pub option_formatting: Formatting,
        pub selected_option_formatting: Formatting,
        pub filter_formatting: Formatting,
        pub not_selected_marker: Marker,
        pub selected_marker: Marker,
    }

    impl Default for SelectionStyle {
        fn default() -> Self {
            SelectionStyle {
                label_style: LabelStyle::default(),
                submitted_formatting: Formatting::default().foreground_color(Color::Green),
                option_formatting: Formatting::default(),
                selected_option_formatting: Formatting::default().bold(),
                filter_formatting: Formatting::default(),
                not_selected_marker: Marker {
                    marker: "  ".into(),
                    formatting: Formatting::default(),
                },
                selected_marker: Marker {
                    marker: "> ".into(),
                    formatting: Formatting::default().bold(),
                },
            }
        }
    }

    impl SelectionStyle {
        pub fn label_style(mut self, l: LabelStyle) -> Self {
            self.label_style = l;
            self
        }

        pub fn submitted_formatting(mut self, f: Formatting) -> Self {
            self.submitted_formatting = f;
            self
        }

        pub fn option_formatting(mut self, f: Formatting) -> Self {
            self.option_formatting = f;
            self
        }

        pub fn not_selected_marker(mut self, m: Marker) -> Self {
            self.not_selected_marker = m;
            self
        }

        pub fn selected_marker(mut self, m: Marker) -> Self {
            self.selected_marker = m;
            self
        }
    }

    impl Marker {
        pub fn print(&self, cmd_buffer: &mut impl CommandBuffer) {
            cmd_buffer.set_formatting(&self.formatting);
            cmd_buffer.print(&self.marker);
            cmd_buffer.reset_formatting();
        }
    }
}

pub mod multiselection {
    use crate::{
        engine::CommandBuffer,
        style::{Color, Formatting, LabelStyle},
    };

    pub struct MultiselectionStyle {
        pub label_style: LabelStyle,
        pub submitted_formatting: Formatting,
        pub filter_formatting: Formatting,
        pub help_message_formatting: Formatting,
        pub marker: Marker,
        pub highlighted_option_formatting: Formatting,
        pub normal_option_formatting: Formatting,
    }

    pub struct Marker {
        pub opening_sign: String,
        pub closing_sign: String,
        pub selection_sign: String,
    }

    impl Default for MultiselectionStyle {
        fn default() -> Self {
            MultiselectionStyle {
                label_style: LabelStyle::default(),
                submitted_formatting: Formatting::default().foreground_color(Color::Green),
                filter_formatting: Formatting::default(),
                help_message_formatting: Formatting::default().foreground_color(Color::DarkGreen),
                marker: Marker {
                    opening_sign: "[".into(),
                    selection_sign: "x".into(),
                    closing_sign: "]".into(),
                },
                highlighted_option_formatting: Formatting::default()
                    .foreground_color(Color::DarkGreen),
                normal_option_formatting: Formatting::default(),
            }
        }
    }

    impl Marker {
        pub fn print(&self, is_selected: bool, commands: &mut impl CommandBuffer) {
            let sign = if is_selected {
                &self.selection_sign
            } else {
                " "
            };
            commands.print(&format!(
                "{}{}{}",
                self.opening_sign, sign, self.closing_sign
            ));
        }
    }

    impl MultiselectionStyle {
        pub fn print_option(
            &self,
            option_text: &str,
            is_selected: bool,
            is_highlighted: bool,
            commands: &mut impl CommandBuffer,
        ) {
            let formatting = if is_highlighted {
                &self.highlighted_option_formatting
            } else {
                &self.normal_option_formatting
            };

            commands.set_formatting(formatting);
            self.marker.print(is_selected, commands);
            commands.print(" ");

            commands.print(option_text);
            commands.reset_formatting();
        }
    }
}
