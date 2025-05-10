use std::io::{self, Read, Write};

use crate::MjbkError;

use super::{lexer::Command, memory::Memory, parser::AstNode};

// Vec<AstNode> を走らせるインタプリタ
pub fn execute(ast: &[AstNode]) -> Result<(), MjbkError> {
    let mut memory = Memory::new();
    execute_block(ast, &mut memory)?;
    Ok(())
}

fn execute_block(block: &[AstNode], memory: &mut Memory) -> Result<(), MjbkError> {
    for node in block {
        match node {
            AstNode::Cmd(cmd) => match cmd {
                Command::Inc => memory.increment(),
                Command::Dec => memory.decrement(),
                Command::MoveRight => memory.move_right(),
                Command::MoveLeft => memory.move_left(),
                Command::Input => {
                    let mut buffer = [0u8];
                    io::stdin().read_exact(&mut buffer)?;
                    memory.set(buffer[0]);
                }
                Command::Output => {
                    let val = memory.get();
                    io::stdout().write_all(&[val])?;
                    io::stdout().flush()?;
                }
                _ => {}
            },
            AstNode::Loop(body) => {
                while memory.get() != 0 {
                    execute_block(body, memory)?;
                }
            }
        }
    }
    Ok(())
}

pub fn run_with_io<W: Write, R: Read>(
    ast: &[AstNode],
    memory: &mut Memory,
    mut output: W,
    mut input: R,
) -> Result<(), MjbkError> {
    execute_block_io(ast, memory, &mut output, &mut input)?;
    Ok(())
}

fn execute_block_io<W: Write, R: Read>(
    block: &[AstNode],
    memory: &mut Memory,
    output: &mut W,
    input: &mut R,
) -> Result<(), MjbkError> {
    for node in block {
        match node {
            AstNode::Cmd(cmd) => match cmd {
                Command::Inc => memory.increment(),
                Command::Dec => memory.decrement(),
                Command::MoveRight => memory.move_right(),
                Command::MoveLeft => memory.move_left(),
                Command::Input => {
                    let mut buffer = [0u8];
                    input.read_exact(&mut buffer)?;
                    memory.set(buffer[0]);
                }
                Command::Output => {
                    let val = memory.get();
                    output.write_all(&[val])?;
                    output.flush()?;
                }
                _ => {}
            },
            AstNode::Loop(body) => {
                while memory.get() != 0 {
                    execute_block_io(body, memory, output, input)?;
                }
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpreter::lexer::tokenize;
    use crate::interpreter::parser::parse;

    #[test]
    fn test_execute_hello_world_capture_output() {
        // let ascii_code = "++++++++++[>+++++++>++++++++++>+++>+<<<<-]>++.>+.+++++++..+++.>++.<<+++++++++++++++.>.+++.------.--------.>+.>.";
        let ascii_code = "繝ｫ繝ｫ繝ｫ繝ｫ繝ｫ繝ｫ繝ｫ繝ｫ繝ｫ繝ｫ血繧ｪ繝ｫ繝ｫ繝ｫ繝ｫ繝ｫ繝ｫ繝ｫ繧ｪ繝ｫ繝ｫ繝ｫ繝ｫ繝ｫ繝ｫ繝ｫ繝ｫ繝ｫ繝ｫ繧ｪ繝ｫ繝ｫ繝ｫ繧ｪ繝ｫ縺縺縺縺隱焚繧ｪ繝ｫ繝ｫ繝?繧ｪ繝ｫ繝?繝ｫ繝ｫ繝ｫ繝ｫ繝ｫ繝ｫ繝ｫ繝?繝?繝ｫ繝ｫ繝ｫ繝?繧ｪ繝ｫ繝ｫ繝?縺縺繝ｫ繝ｫ繝ｫ繝ｫ繝ｫ繝ｫ繝ｫ繝ｫ繝ｫ繝ｫ繝ｫ繝ｫ繝ｫ繝ｫ繝ｫ繝?繧ｪ繝?繝ｫ繝ｫ繝ｫ繝?隱隱隱隱隱隱繝?隱隱隱隱隱隱隱隱繝?繧ｪ繝ｫ繝?繧ｪ繝?";
        let tokens = tokenize(ascii_code).expect("tokenize failed");
        let ast = parse(&tokens).expect("parse failed");

        let mut output = Vec::new();
        let mut input = std::io::empty();
        let mut memory = Memory::new();

        run_with_io(&ast, &mut memory, &mut output, &mut input).expect("run failed");
        let result = String::from_utf8_lossy(&output);

        assert_eq!(result, "Hello World!\n");
    }
}
