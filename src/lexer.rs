use std::{fs::File, path::Path};
use std::io::prelude::*;

pub struct Lexer {
    input: String,
    read_idx: usize
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

#[derive(Debug)]
pub struct Token {
    pub value: TokenValue
}

impl Lexer {
    pub fn from_path(path: &Path) -> Lexer {
        let mut file = match File::open(path) {
            Err(why) => panic!("couldn't open file: {}", why),
            Ok(file) => file,
        };
    
        let mut s = String::new();
        if let Err(why) = file.read_to_string(&mut s) {
            panic!("couldn't read file: {}", why);
        }
        Lexer{input:s, read_idx:0}
    }

    fn accept_pattern(&mut self, pat: &str) -> Option<String> {
        let next = &self.input[self.read_idx..];
        if next.starts_with(pat) {
            self.read_idx += pat.len();
            Some(pat.to_string())
        } else {
            None
        }
    }

    fn accept_identifier(&mut self) -> Option<String> {
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
            let slice = &self.input[self.read_idx .. self.read_idx+end];
            self.read_idx += end;
            Some(slice.to_string())
        } else {
            None
        }
    }

    fn accept_number(&mut self) -> Option<i32> {
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
            let slice = &self.input[self.read_idx .. self.read_idx+end];
            self.read_idx += end;
            Some(slice.parse().unwrap())
        } else {
            None
        }
    }

    fn accept_invalid(&mut self) -> Option<char> {
        let slice = &self.input[self.read_idx..];
        if let Some((i, c)) = slice.char_indices().next() {
            self.read_idx += i;
            Some(c)
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
        self.read_idx += end;
    }
}

impl Iterator for Lexer {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        self.skip_whitespace();
        if let Some(_) = self.accept_pattern(";") {
            return Some(Token{value:TokenValue::Semi});
        } else if let Some(_) = self.accept_pattern("{") {
            return Some(Token{value:TokenValue::LBrace});
        } else if let Some(_) = self.accept_pattern("}") {
            return Some(Token{value:TokenValue::RBrace});
        } else if let Some(_) = self.accept_pattern("->") {
            return Some(Token{value:TokenValue::RArrow});
        } else if let Some(id) = self.accept_identifier() {
            if id == "mnet" {
                return Some(Token{value:TokenValue::KwMNet});
            } else if id == "machine" {
                return Some(Token{value:TokenValue::KwMachine});
            } else if id == "state" {
                return Some(Token{value:TokenValue::KwState});
            } else if id == "initial" {
                return Some(Token{value:TokenValue::KwInitial});
            } else if id == "final" {
                return Some(Token{value:TokenValue::KwFinal});
            } else if id.len() == 1 {
                return Some(Token{value:TokenValue::Ident(id.chars().next().unwrap())});
            } else {
                println!("identifier {} is too long ({})", id, id.len());
                return Some(Token{value:TokenValue::Invalid});
            }
        } else if let Some(num) = self.accept_number() {
            return Some(Token{value:TokenValue::Number(num)});
        } else if let Some(_) = self.accept_invalid() {
            return Some(Token{value:TokenValue::Invalid});
        }
        return None;
    }
}
