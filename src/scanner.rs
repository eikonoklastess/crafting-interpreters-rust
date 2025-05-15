use std::{collections::HashMap, error::Error};

use crate::{error::ErrorReporter, token::*};

pub struct Scanner {
    pub source: Vec<char>,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    pub keywords: HashMap<String, TokenType>,
}

impl Scanner {
    pub fn new(source: &str) -> Self {
        let mut scan = Self {
            source: source.chars().collect(),
            tokens: Vec::<Token>::default(),
            start: 0,
            current: 0,
            line: 1,
            keywords: HashMap::new(),
        };
        scan.keywords.insert("and".to_string(), TokenType::And);
        scan.keywords.insert("class".to_string(), TokenType::Class);
        scan.keywords.insert("else".to_string(), TokenType::Else);
        scan.keywords.insert("false".to_string(), TokenType::False);
        scan.keywords.insert("for".to_string(), TokenType::For);
        scan.keywords.insert("fun".to_string(), TokenType::Fun);
        scan.keywords.insert("if".to_string(), TokenType::If);
        scan.keywords.insert("nil".to_string(), TokenType::Nil);
        scan.keywords.insert("or".to_string(), TokenType::Or);
        scan.keywords.insert("print".to_string(), TokenType::Print);
        scan.keywords
            .insert("return".to_string(), TokenType::Return);
        scan.keywords.insert("super".to_string(), TokenType::Super);
        scan.keywords.insert("this".to_string(), TokenType::This);
        scan.keywords.insert("true".to_string(), TokenType::True);
        scan.keywords.insert("var".to_string(), TokenType::Var);
        scan.keywords.insert("while".to_string(), TokenType::While);

        scan
    }

    pub fn scan_tokens(
        &mut self,
        reporter: &mut ErrorReporter,
    ) -> Result<&Vec<Token>, Box<dyn Error>> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token(reporter);
        }

        self.tokens.push(Token::new(
            TokenType::Eof,
            String::from(""),
            None,
            self.line,
        ));
        Ok(&self.tokens)
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn scan_token(&mut self, reporter: &mut ErrorReporter) {
        let c = self.advance();
        match c {
            '(' => self.add_token(TokenType::LeftParen, None),
            ')' => self.add_token(TokenType::RightParen, None),
            '{' => self.add_token(TokenType::LeftBrace, None),
            '}' => self.add_token(TokenType::RightBrace, None),
            ',' => self.add_token(TokenType::Comma, None),
            '.' => self.add_token(TokenType::Dot, None),
            '-' => self.add_token(TokenType::Minus, None),
            '+' => self.add_token(TokenType::Plus, None),
            ';' => self.add_token(TokenType::Semicolon, None),
            '*' => self.add_token(TokenType::Star, None),
            '!' => {
                let token = {
                    if self.match_token('=') {
                        TokenType::BangEqual
                    } else {
                        TokenType::Bang
                    }
                };
                self.add_token(token, None);
            }
            '=' => {
                let token = {
                    if self.match_token('=') {
                        TokenType::EqualEqual
                    } else {
                        TokenType::Equal
                    }
                };
                self.add_token(token, None);
            }
            '<' => {
                let token = {
                    if self.match_token('=') {
                        TokenType::LessEqual
                    } else {
                        TokenType::Less
                    }
                };
                self.add_token(token, None);
            }
            '>' => {
                let token = {
                    if self.match_token('=') {
                        TokenType::GreaterEqual
                    } else {
                        TokenType::Greater
                    }
                };
                self.add_token(token, None);
            }
            '/' => {
                if self.match_token('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash, None);
                }
            }
            ' ' | '\r' | '\t' => {}
            '\n' => self.line += 1,
            '"' => self.string(reporter),
            '0'..='9' => self.number(),
            c if c.is_ascii_alphabetic() => self.identifier(),
            _ => reporter.error(self.line, "Unexpected character."),
        }
    }

    fn identifier(&mut self) {
        while self.peek().is_ascii_alphanumeric() {
            self.advance();
        }
        let text = String::from_iter(&self.source[self.start..self.current]);
        if let Some(ttype) = self.keywords.get(text.as_str()) {
            self.add_token(ttype.clone(), None);
        } else {
            self.add_token(TokenType::Identifier, None);
        }
    }

    fn number(&mut self) {
        loop {
            let c = self.peek();
            match c {
                '0'..='9' => {
                    self.advance();
                }
                _ => break,
            }
        }

        if self.peek() == '.' {
            let c = self.peek_next();
            match c {
                '0'..='9' => {
                    self.advance();
                    loop {
                        let c = self.peek();
                        match c {
                            '0'..='9' => {
                                self.advance();
                            }
                            _ => break,
                        }
                    }
                }
                _ => (),
            }
        }

        self.add_token(
            TokenType::Number,
            Some(Literal::Number(
                String::from_iter(&self.source[self.start..self.current])
                    .parse::<f64>()
                    .expect("could not parse number"),
            )),
        )
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }
        self.source[self.current + 1]
    }

    fn string(&mut self, reporter: &mut ErrorReporter) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            reporter.error(self.line, "Unterminated string");
            return;
        }

        self.advance();

        let value = String::from_iter(&self.source[self.start + 1..self.current - 1]);
        self.add_token(TokenType::String, Some(Literal::String(value)));
    }
    fn advance(&mut self) -> char {
        let c = self.source[self.current];
        self.current += 1;
        c
    }

    fn add_token(&mut self, ttype: TokenType, lit: Option<Literal>) {
        let text = String::from_iter(&self.source[self.start..self.current]);
        self.tokens.push(Token::new(ttype, text, lit, self.line));
    }

    fn match_token(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.source[self.current] != expected {
            return false;
        }
        self.current += 1;
        return true;
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        self.source[self.current]
    }
}
