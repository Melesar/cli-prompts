mod confirmation;
mod input;
mod options;

pub use confirmation::Confirmation;
pub use input::Input;
pub use options::multiselect::Multiselect;
pub use options::selection::Selection;

use std::io::stdout;

use crate::{
    engine::{Clear, CommandBuffer, CrosstermEngine, Engine},
    input::Key,
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
    fn on_key_pressed(&mut self, key: Key) -> EventOutcome<TOut>;
}

pub trait DisplayPrompt<T> {
    fn display(self) -> Result<T, AbortReason>;
}

impl<T, P> DisplayPrompt<T> for P
where
    P: Prompt<T> + Sized,
{
    fn display(mut self) -> Result<T, AbortReason> {
        let buffer = stdout();
        let mut engine = CrosstermEngine::new(buffer);
        let mut commands = engine.get_command_buffer();

        loop {
            self.draw(&mut commands);
            engine.render(&commands)?;

            let key_pressed = engine.read_key()?;
            match self.on_key_pressed(key_pressed) {
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
                }
                EventOutcome::Abort(reason) => return Err(reason),
            }
        }
    }
}

impl From<std::io::Error> for AbortReason {
    fn from(error: std::io::Error) -> Self {
        AbortReason::Error(error)
    }
}
