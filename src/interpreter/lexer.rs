use crate::error::OlError;

pub enum Command {
    Inc,
    Dec,
    MoveRight,
    MoveLeft,
    Output,
    Input,
    LoopStart,
    LoopEnd,
}

pub fn tokenize(_src: &str) -> Result<Vec<Command>, OlError> {
    todo!()
}
