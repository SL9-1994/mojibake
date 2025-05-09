pub mod error;
// run以外のモジュールは開発時のみ公開 -> 本番環境では秘匿
pub mod interpreter {
    pub mod lexer;
    pub mod memory;
    pub mod parser;
    pub mod run;
}

pub use error::OlError;
pub use interpreter::run::execute;
