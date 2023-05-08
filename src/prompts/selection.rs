use crossterm::{
    cursor::{MoveToNextLine, MoveToPreviousLine, RestorePosition, SavePosition},
    event::KeyCode,
    execute, queue,
    style::{Attribute, Color, Print, SetAttribute, SetForegroundColor},
    terminal::{Clear, ClearType},
};

use crate::output::draw_prompt;

use super::*;

const DEFAULT_OPTIONS_COUNT: usize = 5;

pub struct Selection<T> {
    label: String,
    all_options: Vec<T>,
    transformed_options: Vec<String>,
    filtered_options: Vec<usize>,
    current_selection: usize,
    max_options: usize,
    current_filter: String,
    is_submitted: bool,
}

impl<T> Selection<T> {
    pub fn new<S, I>(label: S, options: I) -> Self
    where
        T: Into<String> + Clone,
        S: Into<String>,
        I: Iterator<Item = T>,
    {
        let options: Vec<T> = options.collect();
        let all_options = options.clone();
        let transformed_options: Vec<String> = options.into_iter().map(|s| s.into()).collect();
        let filtered_options: Vec<usize> = (0..transformed_options.len()).collect();

        Self::new_internal(
            label.into(),
            all_options,
            transformed_options,
            filtered_options,
        )
    }
}

impl<T> Selection<T> {
    pub fn new_with_transformation<S, I, F>(label: S, options: I, transformation: F) -> Self
    where
        S: Into<String>,
        I: Iterator<Item = T>,
        F: Fn(&T) -> String,
    {
        let all_options: Vec<T> = options.collect();
        let transformed_options: Vec<String> = all_options.iter().map(transformation).collect();
        let filtered_options: Vec<usize> = (0..transformed_options.len()).collect();

        Self::new_internal(
            label.into(),
            all_options,
            transformed_options,
            filtered_options,
        )
    }

    pub fn displayed_options_count(mut self, options_count: usize) -> Self {
        self.max_options = options_count;
        self
    }

    fn new_internal(
        label: String,
        all_options: Vec<T>,
        transformed_options: Vec<String>,
        filtered_options: Vec<usize>,
    ) -> Self {
        Selection {
            label,
            all_options,
            transformed_options,
            filtered_options,
            current_selection: 0_usize,
            max_options: DEFAULT_OPTIONS_COUNT,
            current_filter: String::new(),
            is_submitted: false,
        }
    }
}

impl<T> Prompt<T> for Selection<T> {
    fn draw<W: Write>(&self, buffer: &mut W) {
        queue!(buffer, Clear(ClearType::CurrentLine)).unwrap();

        draw_prompt(buffer, &self.label);

        if self.is_submitted {
            self.draw_submitted(buffer);
        } else {
            self.draw_unsubmitted(buffer);
        }

        buffer.flush().unwrap_or_default();
    }

    fn on_event(&mut self, evt: Event) -> EventOutcome<T> {
        match evt {
            Event::Key(key) => match key.code {
                KeyCode::Char(c) => {
                    self.current_filter.push(c);
                    self.filter_options();
                    EventOutcome::Continue
                }
                KeyCode::Backspace if self.current_filter.len() > 0 => {
                    self.current_filter.pop();
                    self.filter_options();
                    EventOutcome::Continue
                }
                KeyCode::Up if self.current_selection > 0 => {
                    self.current_selection -= 1;
                    EventOutcome::Continue
                }
                KeyCode::Down if self.current_selection < self.filtered_options.len() => {
                    self.current_selection += 1;
                    EventOutcome::Continue
                }
                KeyCode::Enter if self.filtered_options.len() > 0 => {
                    self.is_submitted = true;
                    let selected_option_index = self.filtered_options[self.current_selection];
                    let result = self.all_options.remove(selected_option_index);
                    EventOutcome::Done(result)
                }
                _ => EventOutcome::Continue,
            },
            _ => EventOutcome::Continue,
        }
    }
}

impl<T> Selection<T> {
    fn filter_options(&mut self) {
        self.filtered_options.clear();
        for (index, option) in self.transformed_options.iter().enumerate() {
            if option.contains(&self.current_filter) {
                self.filtered_options.push(index);
            }
        }

        self.current_selection = 0;
    }

    fn draw_submitted<W: Write>(&self, buffer: &mut W) {
        queue!(buffer, SetForegroundColor(Color::Green)).unwrap();

        let selected_option_index = self.filtered_options[self.current_selection];
        queue!(
            buffer,
            Print(&self.transformed_options[selected_option_index]),
            Print("\r\n"),
            SetForegroundColor(Color::Reset)
        )
        .unwrap();

        for _ in 0..self.max_options {
            queue!(buffer, Clear(ClearType::CurrentLine), MoveToNextLine(1)).unwrap();
        }

        if let Ok(num_options) = self.max_options.try_into() {
            queue!(buffer, MoveToPreviousLine(num_options)).unwrap();
        }
    }

    fn draw_unsubmitted<W: Write>(&self, buffer: &mut W) {
        queue!(buffer, Print(&self.current_filter), Print("\r\n"),).unwrap();

        let mut start_from = self
            .current_selection
            .checked_sub(self.max_options / 2)
            .unwrap_or(0);
        start_from = start_from.min(
            self.filtered_options
                .len()
                .checked_sub(self.max_options)
                .unwrap_or(0),
        );

        let displayed_option_indices = self
            .filtered_options
            .iter()
            .enumerate()
            .skip(start_from)
            .take(self.max_options);
        let num_displayed_options = displayed_option_indices.len();

        for (selection_index, option_index) in displayed_option_indices {
            let is_selected = selection_index == self.current_selection;
            let prefix = if is_selected { "> " } else { "  " };

            queue!(buffer, Clear(ClearType::CurrentLine)).unwrap();
            if is_selected {
                queue!(buffer, SetAttribute(Attribute::Bold)).unwrap();
            }

            queue!(
                buffer,
                Print(prefix),
                Print(&self.transformed_options[*option_index]),
                SetAttribute(Attribute::Reset),
                Print("\r\n")
            )
            .unwrap();
        }

        for _ in num_displayed_options..self.max_options {
            queue!(buffer, Clear(ClearType::CurrentLine), MoveToNextLine(1)).unwrap();
        }

        if let Ok(num_options) = (self.max_options + 1).try_into() {
            queue!(buffer, MoveToPreviousLine(num_options)).unwrap();
        }
    }
}
