mod input;
mod confirmation;
mod options;

pub use input::Input;
pub use confirmation::Confirmation;
pub use options::selection::Selection;
pub use options::multiselect::Multiselect;

use std::io::stdout;

use crossterm::event::{read, Event};

use crate::{
    engine::{Clear, CommandBuffer, CrosstermEngine, Engine},
    raw_mode::RawMode,
};

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
    fn draw(&self, commands: &mut impl CommandBuffer);
    fn on_event(&mut self, evt: Event) -> EventOutcome<TOut>;
}

pub trait DisplayPrompt<T> {
    fn display(self) -> Result<T, AbortReason>;
}

impl<T, P> DisplayPrompt<T> for P
where
    P: Prompt<T> + Sized,
{
    fn display(mut self) -> Result<T, AbortReason> {
        let _raw = RawMode::ensure();
        let buffer = stdout();
        let mut engine = CrosstermEngine::new(buffer);
        let mut commands = engine.get_command_buffer();

        loop {
            self.draw(&mut commands);
            engine.render(&commands)?;

            match read() {
                Ok(evt) => match self.on_event(evt) {
                    EventOutcome::Done(result) => {
                        commands.clear();
                        self.draw(&mut commands);
                        engine.render(&commands)?;
                        engine.finish_rendering()?;

                        return Ok(result);
                    }
                    EventOutcome::Continue => {
                        commands.clear();
                        continue;
                    },
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
