use std::fs;
use std::io::prelude::*;
use std::{fs::File, path::Path};

pub struct Lexer {
    input: String,
    read_idx: usize,
    read_loc: SourceLocation,
}

struct Fragment<'a> {
    loc: SourceLocation,
    val: &'a str,
}

#[derive(Debug)]
pub enum TokenValue {
    Invalid,
    Number(i32),
    Ident(char),
    Semi,
    LBrace,
    RBrace,
    RArrow,
    KwMNet,
    KwMachine,
    KwState,
    KwInitial,
    KwFinal,
}

#[derive(Debug, Clone, Copy)]
pub struct SourceLocation {
    pub row: usize,
    pub col: usize,
}

impl SourceLocation {
    fn new() -> SourceLocation {
        SourceLocation { row: 0, col: 0 }
    }

    pub fn emit_error(&self, s: &str) {
        eprintln!("{}:{}: error: {}", self.row + 1, self.col + 1, s);
    }
}

#[derive(Debug)]
pub struct Token {
    pub location: SourceLocation,
    pub value: TokenValue,
}

impl Token {
    fn from_frag(frag: &Fragment, value: TokenValue) -> Token {
        Token {
            location: frag.loc,
            value,
        }
    }
}

impl Lexer {
    pub fn from_path(path: &Path) -> Lexer {
        match fs::read_to_string(path) {
            Ok(s) => Lexer {
                input: s,
                read_idx: 0,
                read_loc: SourceLocation::new(),
            },
            Err(why) => panic!("Error file: {}", why),
        }
    }

    fn advance(&mut self, len: usize) -> Fragment {
        let loc = self.read_loc;
        let slice = &self.input[self.read_idx..];
        let mut iter = slice.char_indices();
        let end = loop {
            if let Some((i, c)) = iter.next() {
                if i == len {
                    break i;
                }
                if c == '\n' {
                    self.read_loc.col = 0;
                    self.read_loc.row += 1;
                } else if c != '\r' {
                    self.read_loc.col += 1;
                }
            } else {
                break slice.len();
            }
        };
        let val = &self.input[self.read_idx..self.read_idx + end];
        self.read_idx += end;
        Fragment { loc, val }
    }

    fn accept_pattern(&mut self, pat: &str) -> Option<Fragment> {
        let next = &self.input[self.read_idx..];
        if next.starts_with(pat) {
            Some(self.advance(pat.len()))
        } else {
            None
        }
    }

    fn accept_identifier(&mut self) -> Option<Fragment> {
        let slice = &self.input[self.read_idx..];
        let mut next_iter = slice.char_indices();
        let end = loop {
            if let Some((i, c)) = next_iter.next() {
                let cond = if i == 0 {
                    c.is_ascii_alphabetic()
                } else {
                    c.is_ascii_alphanumeric()
                };
                if !(c == '_' || cond) {
                    break i;
                }
            } else {
                break slice.len();
            }
        };
        if end > 0 {
            Some(self.advance(end))
        } else {
            None
        }
    }

    fn accept_number(&mut self) -> Option<Fragment> {
        let slice = &self.input[self.read_idx..];
        let mut next_iter = slice.char_indices();
        let end = loop {
            if let Some((i, c)) = next_iter.next() {
                if !c.is_ascii_digit() {
                    break i;
                }
            } else {
                break slice.len();
            }
        };
        if end > 0 {
            Some(self.advance(end))
        } else {
            None
        }
    }

    fn accept_invalid(&mut self) -> Option<Fragment> {
        let slice = &self.input[self.read_idx..];
        if let Some((i, _)) = slice.char_indices().next() {
            Some(self.advance(i))
        } else {
            None
        }
    }

    fn skip_whitespace(&mut self) {
        let slice = &self.input[self.read_idx..];
        let mut next_iter = slice.char_indices();
        let end = loop {
            if let Some((i, c)) = next_iter.next() {
                if !c.is_ascii_whitespace() {
                    break i;
                }
            } else {
                break slice.len();
            }
        };
        self.advance(end);
    }
}

impl Iterator for Lexer {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        self.skip_whitespace();
        if let Some(frag) = self.accept_pattern(";") {
            return Some(Token::from_frag(&frag, TokenValue::Semi));
        } else if let Some(frag) = self.accept_pattern("{") {
            return Some(Token::from_frag(&frag, TokenValue::LBrace));
        } else if let Some(frag) = self.accept_pattern("}") {
            return Some(Token::from_frag(&frag, TokenValue::RBrace));
        } else if let Some(frag) = self.accept_pattern("->") {
            return Some(Token::from_frag(&frag, TokenValue::RArrow));
        } else if let Some(frag) = self.accept_identifier() {
            let id = frag.val;
            if id == "mnet" {
                return Some(Token::from_frag(&frag, TokenValue::KwMNet));
            } else if id == "machine" {
                return Some(Token::from_frag(&frag, TokenValue::KwMachine));
            } else if id == "state" {
                return Some(Token::from_frag(&frag, TokenValue::KwState));
            } else if id == "initial" {
                return Some(Token::from_frag(&frag, TokenValue::KwInitial));
            } else if id == "final" {
                return Some(Token::from_frag(&frag, TokenValue::KwFinal));
            } else if id.len() == 1 {
                return Some(Token::from_frag(
                    &frag,
                    TokenValue::Ident(id.chars().next().unwrap()),
                ));
            } else {
                frag.loc.emit_error("identifier longer than one character");
                return Some(Token::from_frag(&frag, TokenValue::Invalid));
            }
        } else if let Some(frag) = self.accept_number() {
            let num = frag.val.parse().unwrap();
            return Some(Token::from_frag(&frag, TokenValue::Number(num)));
        } else if let Some(frag) = self.accept_invalid() {
            return Some(Token::from_frag(&frag, TokenValue::Invalid));
        }
        return None;
    }
}
