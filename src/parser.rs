use std::fs;

use crate::lexer::Lexer;
use crate::token::Token;

#[derive(Debug, Clone)]
pub struct Statement {
    pub token: Token,
    pub value: String,
    pub priority: u8,
    pub line: usize,
    pub target: String,
}

impl Iterator for Statement {
    type Item = Statement;

    fn next(&mut self) -> Option<Statement> {
        Some(self.clone())
    }
}

impl Statement {
    pub fn new(
        token: Token,
        value: String,
        priority: u8,
        line: usize,
        target: String,
    ) -> Statement {
        Statement {
            token,
            value,
            priority,
            line,
            target,
        }
    }

    pub fn empty() -> Statement {
        Statement {
            token: Token::default(),
            value: String::new(),
            priority: 0,
            line: 0,
            target: String::new(),
        }
    }

    pub fn ignore() -> Statement {
        Statement {
            token: Token::Ignore,
            value: String::new(),
            priority: 0,
            line: 0,
            target: String::new(),
        }
    }
}

pub struct Parser<'a> {
    lexer: Lexer<'a>,

    cur_token: Token,
    peek_token: Token,

    cur_priority: u8,
    cur_target: String,

    mod_path: String,
}

impl<'a> Parser<'a> {
    pub fn new(input: &str, mod_path: String) -> Parser {
        let mut lexer = Lexer::new(input);
        let cur_token = lexer.next_token();
        let peek_token = lexer.next_token();

        Parser {
            lexer,

            cur_token,
            peek_token,

            cur_priority: 100,
            cur_target: String::from("Scripts/RoomManager.lua"),

            mod_path,
        }
    }

    fn next_token(&mut self) {
        self.cur_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    pub fn parse(&mut self) -> Vec<Statement> {
        let mut statements = Vec::<Statement>::new();

        loop {
            if self.cur_token == Token::EndOfFile {
                break;
            }

            let stm = match self.cur_token {
                Token::Semicolon => Statement::new(
                    Token::Ignore,
                    String::new(),
                    self.cur_priority,
                    self.lexer.cur_line,
                    self.cur_target.to_string(),
                ),
                Token::Load => self.parse_load(),
                Token::MultilineCommentStart => self.parse_multiline_comment(),
                Token::Comment => self.parse_singleline_comment(),
                Token::To => self.parse_target(),

                Token::Import => self.parse_next_literal(Token::Import),
                Token::Sjson => self.parse_next_literal(Token::Sjson),
                Token::Xml => self.parse_next_literal(Token::Xml),

                Token::Include => {
                    let stm_list = self._include();

                    statements.extend(stm_list);

                    Statement::ignore()
                }

                _ => panic!(
                    "Illegal character on line {} character {} on mod {}.",
                    self.lexer.cur_line, self.lexer.cur_char, self.mod_path
                ),
            };

            if stm.token != Token::Ignore {
                statements.push(stm);
            }

            self.next_token();
        }

        statements
    }

    fn parse_next_literal(&mut self, token: Token) -> Statement {
        let cur_line = self.lexer.cur_line;
        self.next_token();

        Statement::new(
            token,
            get_literal(&self.cur_token),
            self.cur_priority,
            cur_line,
            self.cur_target.to_string(),
        )
    }

    fn parse_target(&mut self) -> Statement {
        self.next_token();
        self.cur_target = get_literal(&self.cur_token);

        Statement::empty()
    }

    fn parse_load(&mut self) -> Statement {
        if self.peek_token != Token::Priority {
            panic!(
                "Syntax error on Load Priority command in line {} on mod {}",
                self.lexer.cur_line, self.mod_path
            );
        }

        self.next_token();
        self.next_token();

        self.cur_priority = get_literal(&self.cur_token).parse().unwrap();

        Statement::empty()
    }

    fn parse_multiline_comment(&mut self) -> Statement {
        while self.cur_token != Token::MultilineCommentEnd {
            self.next_token();
        }

        Statement::ignore()
    }

    fn parse_singleline_comment(&mut self) -> Statement {
        let cur_line = self.lexer.cur_line;

        while self.lexer.cur_line <= cur_line {
            self.next_token();
        }

        Statement::ignore()
    }

    fn _include(&mut self) -> Vec<Statement> {
        let sub_path = self.parse_next_literal(Token::Include).value;

        let content = match fs::read_to_string(self.mod_path.to_string() + "/" + sub_path.as_str())
        {
            Ok(c) => c,
            Err(_) => panic!(
                "Could not find file at line {} in mod {}",
                self.lexer.cur_line, self.mod_path
            ),
        };

        let mut p = Parser::new(content.as_str(), self.mod_path.to_string());

        p.parse()
    }
}

fn get_literal(token: &Token) -> String {
    match token {
        Token::Ident(s) => s.to_string(),
        Token::Integer(s) => s.to_string(),
        Token::String(s) => s.to_string(),

        _ => token.to_string(),
    }
}
