use std::fmt::{Display, Formatter, Result};

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Illegal,
    Ignore,
    EndOfFile,

    // Literals
    Ident(String),
    String(String),
    Integer(String),

    // Operators
    Colon,
    Hyphon,
    MultilineCommentStart,
    MultilineCommentEnd,
    Comment,

    // Delimeter
    Semicolon,

    // Keywords
    Include,
    To,
    Load,
    Priority,
    Import,
    Sjson,
    Xml,
}

impl Default for Token {
    fn default() -> Token {
        Token::Ignore
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{:?}", self)
    }
}

pub fn lookup_indent(ident: &str) -> Token {
    match ident.to_lowercase().as_str() {
        "include" => Token::Include,
        "to" => Token::To,
        "load" => Token::Load,
        "priority" => Token::Priority,
        "import" => Token::Import,
        "sjson" => Token::Sjson,
        "xml" => Token::Xml,
        _ => Token::Ident(ident.to_string()),
    }
}
