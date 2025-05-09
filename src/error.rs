use std::io;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum MjbkError {
    #[error("Invalid character encountered: '{0}'")]
    InvalidChar(char),

    #[error("Mismatched brackets in code")]
    UnmatchedBrackets,

    #[error("I/O error: {0}")]
    Io(#[from] io::Error),

    #[error("Unexpected end of input")]
    UnexpectedEof,

    #[error("Runtime error: {0}")]
    Runtime(String),
}
