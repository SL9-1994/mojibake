use crate::error::MjbkError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

/// マッピングルール：文字列 → Command
static MAP_RULES: &[(&str, Command)] = &[
    ("隱", Command::Dec),
    ("繝ｫ", Command::Inc),
    ("繧ｪ", Command::MoveRight),
    ("縺", Command::MoveLeft),
    ("繝?", Command::Output),
    ("死", Command::Input),
    ("�", Command::LoopStart),
    ("焚", Command::LoopEnd),
];

/// 日本語 DSL をトークンに変換（命令ごとに文字数可変）
pub fn tokenize(src: &str) -> Result<Vec<Command>, MjbkError> {
    let chars: Vec<char> = src.chars().collect();
    let mut tokens = Vec::new();
    let mut i = 0;

    while i < chars.len() {
        let mut matched = false;

        for (pat, cmd) in MAP_RULES {
            let len = pat.chars().count();
            if i + len <= chars.len() && chars[i..i + len].iter().collect::<String>() == *pat {
                tokens.push(*cmd);
                i += len;
                matched = true;
                break;
            }
        }

        if !matched {
            return Err(MjbkError::InvalidChar(chars[i]));
        }
    }

    Ok(tokens)
}

