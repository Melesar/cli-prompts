use std::io::Write;

use crossterm::{
    cursor::{MoveToNextLine, MoveToPreviousLine},
    queue,
    style::Print,
    terminal::{Clear, ClearType},
};

use crate::output::draw_prompt;

use super::Options;

pub trait MultiOptionPrompt<T> {
    fn max_options_count(&self) -> u16;

    fn options(&self) -> &Options<T>;

    fn currently_selected_index(&self) -> usize;

    fn draw_option<W: Write>(
        &self,
        buffer: &mut W,
        option_index: usize,
        option_label: &str,
        is_selected: bool,
    );

    fn draw_header<W: Write>(&self, buffer: &mut W, is_submitted: bool);

    fn draw_multioption<W: Write>(&self, buffer: &mut W, label: &str, is_submitted: bool) {
        queue!(buffer, Clear(ClearType::CurrentLine)).unwrap();

        draw_prompt(buffer, label);
        self.draw_header(buffer, is_submitted);

        queue!(buffer, Print("\r\n")).unwrap();

        if !is_submitted {
            let max_options_count: usize = self.max_options_count().into();
            let mut start_from = self
                .currently_selected_index()
                .checked_sub(max_options_count / 2)
                .unwrap_or(0);
            start_from = start_from.min(
                self.options()
                    .filtered_options()
                    .len()
                    .checked_sub(max_options_count)
                    .unwrap_or(0),
            );

            let displayed_option_indices = self
                .options()
                .filtered_options()
                .iter()
                .enumerate()
                .skip(start_from)
                .take(self.max_options_count().into());

            let num_displayed_options = displayed_option_indices.len();
            for (selection_index, option_index) in displayed_option_indices {
                let is_selected = selection_index == self.currently_selected_index();
                let option_label = &self.options().transformed_options()[*option_index];

                queue!(buffer, Clear(ClearType::CurrentLine)).unwrap();
                self.draw_option(buffer, *option_index, option_label, is_selected);
                queue!(buffer, Print("\r\n")).unwrap();
            }

            for _ in num_displayed_options..self.max_options_count().into() {
                queue!(buffer, Clear(ClearType::CurrentLine), MoveToNextLine(1)).unwrap();
            }

            queue!(buffer, MoveToPreviousLine(self.max_options_count() + 1)).unwrap();
        } else {
            clear_options(buffer, self.max_options_count());
        }

        buffer.flush().unwrap_or_default();
    }
}

fn clear_options<W: Write>(buffer: &mut W, count: u16) {
    for _ in 0..count {
        queue!(buffer, Clear(ClearType::CurrentLine), MoveToNextLine(1)).unwrap();
    }

    queue!(buffer, MoveToPreviousLine(count)).unwrap();
}
