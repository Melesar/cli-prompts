pub mod input;
pub mod confirmation;

mod options;

pub use options::selection;
pub use options::multiselect;

use std::io::Write;

use crossterm::event::{Event, read};

use crate::RawMode;

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
    fn draw<W: Write>(&self, buffer: &mut W);
    fn on_event(&mut self, evt: Event) -> EventOutcome<TOut>;
}

pub fn display_prompt<P, W, T>(mut prompt: P, buffer: &mut W) -> Result<T, AbortReason>
where
    P: Prompt<T>,
    W: Write 
{
    let _raw = RawMode::ensure();
    loop {
        prompt.draw(buffer);
        match read() {
            Ok(evt) => match prompt.on_event(evt) {
                EventOutcome::Done(result) => {
                    prompt.draw(buffer);
                    return Ok(result);
                },
                EventOutcome::Continue => continue,
                EventOutcome::Abort(reason) => return Err(reason),
            },
            Err(error) => return Err(AbortReason::Error(error))
        }
    }
}


