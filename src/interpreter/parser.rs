use crate::error::MjbkError;

use super::lexer::Command;

// ネスト可能な実行単位(抽象構文の最小単位)
#[derive(Debug)]
pub enum AstNode {
    Cmd(Command),
    Loop(Vec<AstNode>),
}

// パース処理：命令トークン列 -> AST(抽象構文木)
pub fn parse(tokens: &[Command]) -> Result<Vec<AstNode>, MjbkError> {
    let mut pos = 0;

    let ast = parse_block(tokens, &mut pos, true)?;

    if pos < tokens.len() {
        return Err(MjbkError::UnexpectedLoopEnd);
    }

    Ok(ast)
}

// ブロック毎に解析
fn parse_block(
    tokens: &[Command],
    pos: &mut usize,
    is_top_level: bool,
) -> Result<Vec<AstNode>, MjbkError> {
    let mut nodes = Vec::new();

    while *pos < tokens.len() {
        match tokens[*pos] {
            Command::LoopStart => {
                *pos += 1;
                let body = parse_block(tokens, pos, false)?;
                nodes.push(AstNode::Loop(body));
            }
            Command::LoopEnd => {
                *pos += 1;
                return Ok(nodes);
            }
            cmd => {
                nodes.push(AstNode::Cmd(cmd));
                *pos += 1;
            }
        }
    }

    // 最上位でループが閉じられていなければエラー
    if !is_top_level {
        Err(MjbkError::UnclosedLoop)
    } else {
        Ok(nodes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use Command::*;

    #[test]
    fn test_parse_simple() {
        let tokens = vec![Inc, MoveRight, LoopStart, Inc, LoopEnd, Output];
        let ast = parse(&tokens).expect("error");
        // ASTの形状
        assert!(matches!(&ast[2], AstNode::Loop(_)));
    }

    #[test]
    fn test_parse_unclosed_loop() {
        let tokens = vec![Inc, LoopStart, Inc];
        let result = parse(&tokens);
        assert!(result.is_err());
    }
}
