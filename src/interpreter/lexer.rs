use crate::error::MtError;

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

pub fn tokenize(_src: &str) -> Result<Vec<Command>, MtError> {
    todo!()
}
