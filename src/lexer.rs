use std::{iter::Peekable, str::Chars};

use crate::{lookup_indent, Token};

pub struct Lexer<'a> {
    pub cur_line: usize,
    pub cur_char: usize,

    input: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &str) -> Lexer {
        Lexer {
            cur_line: 1,
            cur_char: 1,

            input: input.chars().peekable(),
        }
    }

    fn next(&mut self) -> Option<char> {
        self.cur_char += 1;

        self.input.next()
    }

    fn peek(&mut self) -> Option<&char> {
        self.input.peek()
    }

    fn peek_char_eq(&mut self, ch: char) -> bool {
        match self.peek() {
            Some(&peek_cha) => peek_cha == ch,
            None => false,
        }
    }

    fn skip_spaces(&mut self) {
        while let Some(&c) = self.peek() {
            if !c.is_whitespace() {
                break;
            }

            if c == '\n' {
                self.cur_line += 1;
                self.cur_char = 1;
            }

            self.next();
        }
    }

    fn peek_is_letter(&mut self) -> bool {
        match self.peek() {
            Some(&ch) => is_letter(ch),
            None => false,
        }
    }

    fn read_indentifier(&mut self, first: char) -> String {
        let mut ident = String::new();
        ident.push(first);

        while self.peek_is_letter() {
            match self.next() {
                Some(ch) => ident.push(ch),
                None => continue,
            }
        }

        ident
    }

    fn read_number(&mut self, first: char) -> String {
        let mut number = String::new();
        number.push(first);

        while let Some(&c) = self.peek() {
            if !c.is_numeric() {
                break;
            }

            match self.next() {
                Some(ch) => number.push(ch),
                None => continue,
            }
        }

        number
    }

    fn read_string(&mut self) -> String {
        let mut s = String::new();

        while !self.peek_char_eq('"') {
            match self.next() {
                Some(ch) => s.push(ch),
                None => continue,
            }
        }

        if !self.peek_char_eq('"') {
            panic!("String is not closed!");
        }

        self.next();

        s
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_spaces();

        match self.next() {
            Some('-') => {
                if self.peek_char_eq(':') {
                    self.next();
                    Token::MultilineCommentStart
                } else {
                    Token::Illegal
                }
            }
            Some(':') => {
                if self.peek_char_eq(':') {
                    self.next();
                    Token::Comment
                } else if self.peek_char_eq('-') {
                    self.next();
                    Token::MultilineCommentEnd
                } else {
                    Token::Illegal
                }
            }
            Some(';') => Token::Semicolon,
            Some('"') => Token::String(self.read_string()),

            Some(ch @ _) => {
                if is_letter(ch) {
                    let literal = self.read_indentifier(ch);
                    lookup_indent(literal.as_str())
                } else if ch.is_numeric() {
                    Token::Integer(self.read_number(ch))
                } else {
                    Token::Illegal
                }
            }

            None => Token::EndOfFile,
        }
    }
}

// is_letter checks whether a char is a valid alphabetic character or an underscore
fn is_letter(ch: char) -> bool {
    ch.is_alphabetic() || ch == '_'
}
