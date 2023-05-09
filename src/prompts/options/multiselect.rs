use std::io::Write;

use crossterm::{
    event::{Event, KeyCode},
    queue,
    style::{Attribute, Color, Print, SetAttribute, SetForegroundColor},
};

use crate::prompts::{EventOutcome, Prompt};

use super::{multi_option_prompt::MultiOptionPrompt, Options};

const DEFAUTL_MAX_OPTIONS: u16 = 5;
const DEFAULT_HELP_MESSAGE: &str = "Space to select, enter to submit";

pub struct Multiselect<T> {
    label: String,
    options: Options<T>,
    selected_options: Vec<usize>,
    help_message: Option<String>,
    max_displayed_options: u16,
    currently_selected_index: usize,
    is_submitted: bool,
    filter: String,
}

impl<T> Multiselect<T>
where
    T: Into<String> + Clone,
{
    pub fn new<S, I>(label: S, options: I) -> Self
    where
        S: Into<String>,
        I: Iterator<Item = T>,
    {
        let options = Options::from_iter(options);
        Self::new_internal(label.into(), options)
    }
}

impl<T> Multiselect<T> {
    pub fn new_transformed<S, I, F>(label: S, options: I, transformation: F) -> Self
    where
        S: Into<String>,
        I: Iterator<Item = T>,
        F: Fn(&T) -> String,
    {
        let options = Options::from_iter_transformed(options, transformation);
        Self::new_internal(label.into(), options)
    }

    pub fn help_message<S: Into<String>>(mut self, message: S) -> Self {
        self.help_message = Some(message.into());
        self
    }

    pub fn dont_display_help_message(mut self) -> Self {
        self.help_message = None;
        self
    }

    pub fn max_displayed_options(mut self, max_options: u16) -> Self {
        self.max_displayed_options = max_options;
        self
    }
}

impl<T> MultiOptionPrompt<T> for Multiselect<T> {
    fn max_options_count(&self) -> u16 {
        self.max_displayed_options
    }

    fn options(&self) -> &Options<T> {
        &self.options
    }

    fn currently_selected_index(&self) -> usize {
        self.currently_selected_index
    }

    fn draw_option<W: Write>(
        &self,
        buffer: &mut W,
        option_index: usize,
        option_label: &str,
        is_selected: bool,
    ) -> Result<(), std::io::Error> {
        if is_selected {
            queue!(buffer, SetForegroundColor(Color::DarkGreen))?;
        }
        queue!(buffer, Print("["))?;
        if self.selected_options.contains(&option_index) {
            queue!(buffer, Print("x"))?;
        } else {
            queue!(buffer, Print(" "))?;
        }
        queue!(buffer, Print("] "))?;
        queue!(buffer, Print(option_label))?;

        if is_selected {
            queue!(buffer, SetForegroundColor(Color::Reset))?;
        }

        Ok(())
    }

    fn draw_header<W: Write>(&self, buffer: &mut W, is_submitted: bool) -> Result<(), std::io::Error> {
        if is_submitted {
            queue!(buffer, SetForegroundColor(Color::DarkGreen))?;
            for (i, selected_index) in self.selected_options.iter().enumerate() {
                let selected_option = &self.options.transformed_options()[*selected_index];
                queue!(buffer, Print(selected_option))?;

                if i < self.selected_options.len() - 1 {
                    queue!(buffer, Print(", "))?;
                }
            }
            queue!(buffer, SetForegroundColor(Color::Reset))?;
        } else {
            queue!(buffer, Print(&self.filter), Print(" "))?;
            if let Some(help_message) = self.help_message.as_ref() {
                queue!(
                    buffer,
                    SetForegroundColor(Color::DarkGreen),
                    Print("["),
                    Print(help_message),
                    Print("]"),
                    SetForegroundColor(Color::Reset)
                )?;
            }
        }

        Ok(())
    }
}

impl<T> Prompt<Vec<T>> for Multiselect<T> {
    fn draw<W: std::io::Write>(&self, buffer: &mut W) -> Result<(), std::io::Error>{
        self.draw_multioption(buffer, &self.label, self.is_submitted)
    }

    fn on_event(&mut self, evt: Event) -> EventOutcome<Vec<T>> {
        match evt {
            Event::Key(key) => match key.code {
                KeyCode::Up if self.currently_selected_index > 0 => {
                    self.currently_selected_index -= 1;
                    EventOutcome::Continue
                }
                KeyCode::Down
                    if self.currently_selected_index
                        < self.options.filtered_options().len() - 1 =>
                {
                    self.currently_selected_index += 1;
                    EventOutcome::Continue
                }
                KeyCode::Char(c) => {
                    if c == ' ' {
                        let selected_option_index =
                            self.options.filtered_options()[self.currently_selected_index];
                        let existing_value_index = self
                            .selected_options
                            .iter()
                            .enumerate()
                            .find(|&x| *x.1 == selected_option_index)
                            .map(|x| x.0);

                        if let Some(i) = existing_value_index {
                            self.selected_options.remove(i);
                        } else {
                            self.selected_options.push(selected_option_index);
                        }

                        if self.filter.len() > 0 {
                            self.filter.clear();
                            self.options.filter(&self.filter);
                            self.currently_selected_index = 0;
                        }
                        EventOutcome::Continue
                    } else {
                        self.filter.push(c);
                        self.options.filter(&self.filter);
                        self.currently_selected_index = 0;
                        EventOutcome::Continue
                    }
                }
                KeyCode::Backspace if self.filter.len() > 0 => {
                    self.filter.pop();
                    self.options.filter(&self.filter);
                    self.currently_selected_index = 0;
                    EventOutcome::Continue
                }
                KeyCode::Enter if self.selected_options.len() > 0 => {
                    self.is_submitted = true;
                    self.selected_options.sort();

                    let mut result = vec![];
                    for selected_option_index in self.selected_options.iter().rev() {
                        let selected_option = self
                            .options
                            .all_options_mut()
                            .remove(*selected_option_index);
                        result.push(selected_option);
                    }

                    EventOutcome::Done(result)
                }
                _ => EventOutcome::Continue,
            },
            _ => EventOutcome::Continue,
        }
    }
}

impl<T> Multiselect<T> {
    fn new_internal(label: String, options: Options<T>) -> Self {
        Multiselect {
            label,
            options,
            selected_options: vec![],
            help_message: Some(DEFAULT_HELP_MESSAGE.into()),
            max_displayed_options: DEFAUTL_MAX_OPTIONS,
            currently_selected_index: 0,
            is_submitted: false,
            filter: String::new(),
        }
    }
}
