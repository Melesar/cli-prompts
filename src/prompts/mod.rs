mod confirmation;
mod input;
mod options;

pub use input::Input;
pub use confirmation::Confirmation;
pub use options::multiselect::Multiselect;
pub use options::selection::Selection;

use std::io::Write;

use crossterm::event::{read, Event};

use crate::raw_mode::RawMode;

#[derive(Debug)]
pub enum AbortReason {
    Interrupt,
    Error(std::io::Error),
}

#[derive(Debug)]
pub enum EventOutcome<T> {
    Done(T),
    Continue,
    Abort(AbortReason),
}

pub trait Prompt<TOut> {
    fn draw<W: Write>(&self, buffer: &mut W) -> Result<(), std::io::Error>;
    fn on_event(&mut self, evt: Event) -> EventOutcome<TOut>;
}

pub trait DisplayPrompt<T> {
    fn display<W: Write>(self, buffer: &mut W) -> Result<T, AbortReason>;
}

impl<T, P> DisplayPrompt<T> for P
where
    P: Prompt<T> + Sized,
{
    fn display<W: Write>(mut self, buffer: &mut W) -> Result<T, AbortReason> {
        let _raw = RawMode::ensure();
        loop {
            self.draw(buffer)?;
            match read() {
                Ok(evt) => match self.on_event(evt) {
                    EventOutcome::Done(result) => {
                        self.draw(buffer)?;
                        return Ok(result);
                    }
                    EventOutcome::Continue => continue,
                    EventOutcome::Abort(reason) => return Err(reason),
                },
                Err(error) => return Err(error.into()),
            }
        }
    }
}

impl From<std::io::Error> for AbortReason {
    fn from(error: std::io::Error) -> Self {
        AbortReason::Error(error)
    }
}
