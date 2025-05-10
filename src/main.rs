use std::{
    fs::File,
    io::{BufReader, Read},
};

use clap::Parser;
use mojibake::{
    MjbkError,
    interpreter::{lexer::tokenize, memory::Memory, parser::parse, run::run_with_io},
};

// Brainfuck 派生言語 MJBK の実行バイナリ
#[derive(Debug, PartialEq, Eq, Clone, Parser)]
#[clap(author, version, about = "Minimal Japanese Mojibake Brainfuck-style interpreter type language.", long_about = None)]
struct Cli {
    /// 実行する .mjbk ファイルへのパス
    #[arg(value_name = "FILE")]
    file: std::path::PathBuf,
}

fn main() -> Result<(), MjbkError> {
    let cli = Cli::parse();

    let file = File::open(&cli.file).map_err(|e| MjbkError::Io(e))?;
    let mut reader = BufReader::new(file);
    let mut code = String::new();
    reader.read_to_string(&mut code)?;

    let tokens = tokenize(&code)?;
    let ast = parse(&tokens)?;

    let mut std_output = std::io::stdout().lock();
    let mut std_input = std::io::stdin().lock();
    let mut memory = Memory::new();

    run_with_io(&ast, &mut memory, &mut std_output, &mut std_input)?;

    Ok(())
}
