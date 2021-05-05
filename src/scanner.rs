use std::collections::HashMap;

use lazy_static::lazy_static;

use crate::token::{Literal, Token, TokenType};
use crate::utils;

lazy_static! {
    static ref KEYWORDS: HashMap<&'static str, TokenType> = {
        let mut m = HashMap::new();

        m.insert("and", TokenType::And);
        m.insert("class", TokenType::Class);
        m.insert("else", TokenType::Else);
        m.insert("false", TokenType::False);
        m.insert("for", TokenType::For);
        m.insert("Fun", TokenType::Fun);
        m.insert("if", TokenType::If);
        m.insert("nil", TokenType::Nil);
        m.insert("or", TokenType::Or);
        m.insert("print", TokenType::Print);
        m.insert("return", TokenType::Return);
        m.insert("super", TokenType::Super);
        m.insert("this", TokenType::This);
        m.insert("true", TokenType::True);
        m.insert("var", TokenType::Var);
        m.insert("while", TokenType::While);

        m
    };
}

#[derive(Debug)]
pub struct Scanner {
    chars: Vec<char>,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: u32,
}

impl Scanner {
    pub fn new(source: &str) -> Scanner {
        Scanner {
            chars: source.chars().collect(),
            tokens: Vec::<Token>::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;

            self.scan_token();
        }

        self.tokens
            .push(Token::new(TokenType::EOF, "".to_string(), None, self.line));

        self.tokens.iter().cloned().collect()
    }

    fn is_at_end(&self) -> bool {
        return self.current >= self.chars.len();
    }

    fn scan_token(&mut self) {
        match self.advance() {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftParen),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Dot),
            '.' => self.add_token(TokenType::Comma),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::Semicolon),
            '*' => self.add_token(TokenType::Plus),
            // Operators
            '!' => {
                if self.next_if_eq('=') {
                    self.add_token(TokenType::BangEqual)
                } else {
                    self.add_token(TokenType::Bang)
                }
            }
            '=' => {
                if self.next_if_eq('=') {
                    self.add_token(TokenType::EqualEqual);
                } else {
                    self.add_token(TokenType::Equal);
                }
            }
            '<' => {
                if self.next_if_eq('=') {
                    self.add_token(TokenType::LessEqual)
                } else {
                    self.add_token(TokenType::Less)
                }
            }
            '>' => {
                if self.next_if_eq('=') {
                    self.add_token(TokenType::GreaterEqual)
                } else {
                    self.add_token(TokenType::Greater)
                }
            }
            '/' => {
                if self.next_if_eq('/') {
                    while self.peek() != Some('\n') {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash)
                }
            }
            ' ' | '\r' | '\t' => {}
            '\n' => {
                self.line += 1;
            }
            '"' => self.string(),
            c if c.is_ascii_digit() => {
                self.number();
            }
            c if c.is_alphabetic() => {
                self.identifier();
            }
            _ => {
                utils::error(self.line, "Unexpected characters.");
            }
        }
    }

    fn advance(&mut self) -> char {
        let ch = self.chars[self.current];
        self.current += 1;
        ch
    }

    fn add_token(&mut self, kind: TokenType) {
        self.tokens.push(Token::new(
            kind,
            substr(&self.chars, self.start, self.current),
            None,
            self.line,
        ))
    }

    fn add_literal(&mut self, kind: TokenType, literal: Option<Literal>) {
        self.tokens.push(Token::new(
            kind,
            substr(&self.chars, self.start, self.current),
            literal,
            self.line,
        ))
    }

    fn next_if_eq(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.chars[self.current] != expected {
            return false;
        }

        self.current += 1;
        true
    }

    fn peek(&self) -> Option<char> {
        if self.is_at_end() {
            None
        } else {
            Some(self.chars[self.current])
        }
    }

    fn peek_next(&self) -> Option<char> {
        let idx = self.current + 1;
        if idx + 1 >= self.chars.len() {
            None
        } else {
            Some(self.chars[idx])
        }
    }

    fn string(&mut self) {
        while let Some(ch) = self.peek() {
            if ch == '"' {
                break;
            }
            if ch == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            utils::error(self.line, "Unterminated string.");
            return;
        }

        self.advance();

        let value: String = substr(&self.chars, self.start + 1, self.current - 1);

        self.add_literal(TokenType::String, Some(Literal::String(value)));
    }

    fn number(&mut self) {
        while self.peek().map_or(false, |c| c.is_ascii_digit()) {
            self.advance();
        }

        if self.peek() == Some('.') && self.peek_next().map_or(false, |c| c.is_ascii_digit()) {
            self.advance();

            while self.peek().map_or(false, |c| c.is_ascii_digit()) {
                self.advance();
            }
        }

        let value = substr(&self.chars, self.start, self.current)
            .parse::<f64>()
            .unwrap_or_default();

        self.add_literal(TokenType::Number, Some(Literal::Number(value)))
    }

    fn identifier(&mut self) {
        while self.peek().map_or(false, |c| c.is_alphanumeric()) {
            self.advance();
        }

        let text = substr(&self.chars, self.start, self.current);
        let kind = KEYWORDS.get(&text.as_str());

        match kind {
            Some(&k) => self.add_token(k),
            None => self.add_literal(TokenType::Identifier, Some(Literal::Identifier(text))),
        }
    }
}

fn substr(chars: &Vec<char>, start: usize, end: usize) -> String {
    chars[start..end].iter().collect()
}

pub fn run(source: &str) -> std::io::Result<()> {
    let mut scanner = Scanner::new(source);

    let tokens = scanner.scan_tokens();

    for token in tokens {
        println!("{}", token)
    }

    Ok(())
}
