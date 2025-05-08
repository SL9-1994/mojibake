use crate::error::MtError;

use super::lexer::Command;

pub enum AstNode {
    Cmd(Command),
    Loop(Vec<AstNode>),
}

pub fn parse(_tokens: &[Command]) -> Result<Vec<AstNode>, MtError> {
    todo!()
}
