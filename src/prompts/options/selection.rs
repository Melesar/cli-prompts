use crossterm::event::{Event, KeyCode};

use crate::{engine::CommandBuffer, prompts::{options::Options, EventOutcome, AbortReason, Prompt}, style::SelectionStyle};

use super::multioption_prompt::MultiOptionPrompt;

const DEFAULT_OPTIONS_COUNT: u16 = 5;

pub struct Selection<T> {
    label: String,
    options: Options<T>,
    current_selection: usize,
    max_options: u16,
    current_filter: String,
    is_submitted: bool,
    style: SelectionStyle,
}

impl<T> Selection<T> {
    pub fn new<S, I>(label: S, options: I) -> Self
    where
        T: Into<String> + Clone,
        S: Into<String>,
        I: Iterator<Item = T>,
    {
        let options = Options::from_iter(options);
        Self::new_internal(label.into(), options)
    }
}

impl<T> Selection<T> {
    pub fn new_with_transformation<S, I, F>(label: S, options: I, transformation: F) -> Self
    where
        S: Into<String>,
        I: Iterator<Item = T>,
        F: Fn(&T) -> String,
    {
        let options = Options::from_iter_transformed(options, transformation);
        Self::new_internal(label.into(), options)
    }

    pub fn displayed_options_count(mut self, options_count: u16) -> Self {
        self.max_options = options_count;
        self
    }

    pub fn style(mut self, style: SelectionStyle) -> Self {
        self.style = style;
        self
    }

    fn new_internal(label: String, options: Options<T>) -> Self {
        Selection {
            label,
            options,
            current_selection: 0_usize,
            max_options: DEFAULT_OPTIONS_COUNT,
            current_filter: String::new(),
            is_submitted: false,
            style: SelectionStyle::default(),
        }
    }
}

impl<T> MultiOptionPrompt<T> for Selection<T> {
    fn max_options_count(&self) -> u16 {
        self.max_options
    }

    fn options(&self) -> &Options<T> {
        &self.options
    }

    fn currently_selected_index(&self) -> usize {
        self.current_selection
    }

    fn draw_option(
        &self,
        _: usize,
        option_label: &str,
        is_selected: bool,
        cmd_buffer: &mut impl CommandBuffer,
    ) {
        if is_selected {
            self.style.selected_marker.print_cmd(cmd_buffer);
            self.style .selected_option_formatting.print_cmd(option_label, cmd_buffer);
        } else {
            self.style.not_selected_marker.print_cmd(cmd_buffer);
            self.style.option_formatting.print_cmd(option_label, cmd_buffer)
        }
    }

    fn draw_header(
        &self,
        commands: &mut impl CommandBuffer,
        is_submitted: bool,
    )  {
        if is_submitted {
            let selected_option_index = self.options.filtered_options()[self.current_selection];
            let selected_option = &self.options.transformed_options()[selected_option_index];
            self.style
                .submitted_formatting
                .print_cmd(selected_option, commands);
        } else {
            self.style
                .filter_formatting
                .print_cmd(&self.current_filter, commands);
        }
    }
}

impl<T> Prompt<T> for Selection<T> {
    fn draw(&self, commands: &mut impl CommandBuffer) {
        self.draw_multioption(
            &self.label,
            self.is_submitted,
            &self.style.label_style,
            commands
        )
    }

    fn on_event(&mut self, evt: Event) -> EventOutcome<T> {
        match evt {
            Event::Key(key) => match key.code {
                KeyCode::Char(c) => {
                    self.current_filter.push(c);
                    self.options.filter(&self.current_filter);
                    self.current_selection = 0;
                    EventOutcome::Continue
                }
                KeyCode::Backspace if self.current_filter.len() > 0 => {
                    self.current_filter.pop();
                    self.options.filter(&self.current_filter);
                    self.current_selection = 0;
                    EventOutcome::Continue
                }
                KeyCode::Up if self.current_selection > 0 => {
                    self.current_selection -= 1;
                    EventOutcome::Continue
                }
                KeyCode::Down
                    if self.current_selection < self.options.filtered_options().len() - 1 =>
                {
                    self.current_selection += 1;
                    EventOutcome::Continue
                }
                KeyCode::Enter if self.options.filtered_options().len() > 0 => {
                    self.is_submitted = true;
                    let selected_option_index =
                        self.options.filtered_options()[self.current_selection];
                    let result = self.options.all_options_mut().remove(selected_option_index);
                    EventOutcome::Done(result)
                }
                KeyCode::Esc => EventOutcome::Abort(AbortReason::Interrupt),
                _ => EventOutcome::Continue,
            },
            _ => EventOutcome::Continue,
        }
    }
}
