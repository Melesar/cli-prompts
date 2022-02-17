use crate::error::Result;
use crate::{draw_promt, RawMode};
use std::thread::current;
use std::{fmt::Display, io::Write};

const DEFAULT_OPTIONS_COUNT: usize = 5;

use crossterm::{
    cursor::{
        self, MoveRight, MoveTo, MoveToNextLine, MoveToPreviousLine, RestorePosition, SavePosition,
    },
    event::{read, Event, KeyCode},
    execute, queue,
    style::{Attribute, Color, Print, ResetColor, SetAttribute, SetForegroundColor},
    terminal::{Clear, ClearType},
};

pub fn select_one<S, I, T>(label: S, options: I) -> Selection<I, T>
where
    S: Into<String>,
    I: Iterator<Item = T>,
{
    Selection::new(label.into(), options)
}

pub struct Selection<I, T>
where
    I: Iterator<Item = T>,
{
    label: String,
    options: I,
    max_options: usize,
}

impl<I> Selection<I, String>
where
    I: Iterator<Item = String>,
{
    pub fn show<W: Write>(self, buffer: &mut W) -> Result<String> {
        self.show_transformed(buffer, |s| s.into())
    }
}

impl<I, T> Selection<I, T>
where
    I: Iterator<Item = T>,
{
    fn show_transformed<W, F>(self, buffer: &mut W, transformation: F) -> Result<T>
    where
        F: FnMut(&T) -> String,
        W: Write,
    {
        let mut all_options: Vec<T> = self.options.collect();
        let mut transformed_options: Vec<String> = all_options.iter().map(transformation).collect();
        let mut current_filter = String::new();
        let mut current_options: Vec<(usize, &String)> =
            transformed_options.iter().enumerate().collect();
        let mut current_selection = Some(0_usize);

        draw_promt(buffer, &self.label, &None::<String>)?;

        let initial_cursor_position = crossterm::cursor::position().unwrap_or_default();
        let options_to_draw = std::cmp::min(self.max_options, transformed_options.len());
        for (idx, option) in transformed_options.iter().take(options_to_draw).enumerate() {
            let prefix = current_selection.map_or(" ", |s| if s == idx { ">" } else { " " });
            print!("\n{} {}", prefix, option);
        }

        execute!(
            buffer,
            MoveToPreviousLine(options_to_draw as u16),
            MoveRight(initial_cursor_position.0),
            SavePosition
        )?;

        let initial_cursor_position = cursor::position().unwrap_or_default();
        let move_to_start = MoveTo(initial_cursor_position.0, initial_cursor_position.1);

        let _raw_mode = RawMode::ensure();

        let selected_option: usize;
        loop {
            match read().map_err(|e| crate::error::Error::IoError(e))? {
                Event::Key(k) => match k.code {
                    KeyCode::Char(c) => {
                        current_filter.push(c);
                        execute!(buffer, Print(c), SavePosition)?;
                        current_options = apply_filter(&current_filter, &transformed_options);
                        current_selection = update_selection(&current_options, current_selection);

                        draw_options(
                            buffer,
                            &current_selection,
                            &current_options,
                            self.max_options,
                        )?;
                    }
                    KeyCode::Backspace => {
                        current_filter.pop();
                        execute!(
                            buffer,
                            move_to_start,
                            Clear(ClearType::UntilNewLine),
                            Print(&current_filter),
                            SavePosition
                        )?;
                        current_options = apply_filter(&current_filter, &transformed_options);
                        current_selection = update_selection(&current_options, current_selection);

                        draw_options(
                            buffer,
                            &current_selection,
                            &current_options,
                            self.max_options,
                        )?;
                    }
                    KeyCode::Down => {
                        current_selection = current_selection
                            .and_then(|v| Some(std::cmp::min(v + 1, current_options.len() - 1)));
                        draw_options(
                            buffer,
                            &current_selection,
                            &current_options,
                            self.max_options,
                        )?;
                    }
                    KeyCode::Up => {
                        current_selection =
                            current_selection.and_then(|v| v.checked_sub(1).or(Some(0)));
                        draw_options(
                            buffer,
                            &current_selection,
                            &current_options,
                            self.max_options,
                        )?;
                    }
                    KeyCode::Enter => {
                        if let Some(s) = current_selection {
                            selected_option = current_options[s].0;
                            break;
                        }
                    }
                    _ => (),
                },
                _ => (),
            }
        }

        queue!(buffer, RestorePosition)?;
        for _ in 0..current_options.len() {
            queue!(buffer, MoveToNextLine(1), Clear(ClearType::CurrentLine))?;
        }

        let selection = transformed_options.remove(selected_option);
        queue!(buffer, move_to_start, Clear(ClearType::UntilNewLine))?;
        queue!(
            buffer,
            SetForegroundColor(Color::DarkCyan),
            Print(selection),
            ResetColor
        )?;
        queue!(buffer, MoveToNextLine(1))?;
        buffer.flush()?;

        Ok(all_options.remove(selected_option))
    }

    fn new(label: String, options: I) -> Self {
        Selection {
            label,
            options,
            max_options: DEFAULT_OPTIONS_COUNT,
        }
    }

    pub fn displayed_options_count(mut self, count: usize) -> Self {
        self.max_options = count;
        self
    }
}

fn draw_options<U: Display, W: Write>(
    buffer: &mut W,
    selection: &Option<usize>,
    current_options: &[(usize, &U)],
    max_visible_options: usize,
) -> Result<()> {
    queue!(buffer, RestorePosition, MoveToNextLine(1))?;
    let mut start_from = selection.map_or(0, |s| s.checked_sub(max_visible_options / 2).unwrap_or(0));
    start_from = start_from.min(current_options.len().checked_sub(max_visible_options).unwrap_or(0));

    let iter = current_options
        .iter()
        .enumerate()
        .skip(start_from)
        .take(max_visible_options);

    for (selection_index, (_, option)) in iter {
        let is_selected = selection.filter(|s| *s == selection_index).is_some();
        let prefix = if is_selected { "> " } else { "  " };
        queue!(buffer, Clear(ClearType::CurrentLine))?;
        if is_selected {
            queue!(buffer, SetAttribute(Attribute::Bold))?;
        }
        queue!(
            buffer,
            Print(prefix),
            Print(option),
            SetAttribute(Attribute::Reset),
            MoveToNextLine(1)
        )?;
    }

    for _ in current_options.len()..max_visible_options {
        queue!(buffer, Clear(ClearType::CurrentLine), MoveToNextLine(1))?;
    }

    queue!(buffer, RestorePosition)?;
    buffer.flush()?;

    Ok(())
}

fn apply_filter<'a>(filter: &str, all_options: &'a [String]) -> Vec<(usize, &'a String)> {
    all_options
        .iter()
        .enumerate()
        .filter(|(_, s)| s.contains(filter))
        .collect()
}

fn update_selection<U: Display>(
    current_options: &[(usize, &U)],
    current_selection: Option<usize>,
) -> Option<usize> {
    if current_options.len() == 0 {
        None
    } else {
        Some(current_selection.map_or(0, |selection| {
            std::cmp::min(selection, current_options.len() - 1)
        }))
    }
}
