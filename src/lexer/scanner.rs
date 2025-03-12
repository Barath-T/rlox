use std::collections::HashMap;

use super::token::{Token, TokenType};
use crate::utils;

pub struct Scanner<'a> {
    source: &'a str,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: u32,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Scanner<'a> {
        return Self {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        };
    }

    pub fn scan_tokens(&mut self, had_err: &mut bool) -> &Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token(had_err);
        }

        self.tokens
            .push(Token::new(TokenType::Eof, String::new(), self.line));
        &self.tokens
    }

    fn scan_token(&mut self, had_err: &mut bool) {
        // Mapping literals to keywords?
        let keywords_map: HashMap<&str, TokenType> = HashMap::from([
            ("and", TokenType::And),
            ("class", TokenType::Class),
            ("else", TokenType::Else),
            ("false", TokenType::False),
            ("fun", TokenType::Fun),
            ("for", TokenType::For),
            ("if", TokenType::If),
            ("nil", TokenType::Nil),
            ("or", TokenType::Or),
            ("print", TokenType::Print),
            ("return", TokenType::Return),
            ("super", TokenType::Super),
            ("this", TokenType::This),
            ("true", TokenType::True),
            ("var", TokenType::Var),
            ("while", TokenType::While),
        ]);

        match self.advance() {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::Semicolon),
            '*' => self.add_token(TokenType::Star),

            '!' => match self.expected_match('=') {
                true => self.add_token(TokenType::BangEqual),
                false => self.add_token(TokenType::Bang),
            },
            '=' => match self.expected_match('=') {
                true => self.add_token(TokenType::EqualEqual),
                false => self.add_token(TokenType::Equal),
            },
            '<' => match self.expected_match('=') {
                true => self.add_token(TokenType::LessEqual),
                false => self.add_token(TokenType::Less),
            },
            '>' => match self.expected_match('=') {
                true => self.add_token(TokenType::GreaterEqual),
                false => self.add_token(TokenType::Greater),
            },
            '/' => match self.expected_match('/') {
                true => {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                }
                false => self.add_token(TokenType::Slash),
            },

            ' ' | '\r' | '\t' => (),
            '\n' => self.line += 1,

            '"' => match self.string() {
                Ok(string_literal) => self.add_token(TokenType::String(string_literal)),
                Err(err_msg) => utils::lex_error(self.line, err_msg, had_err),
            },

            '0'..='9' => {
                let number: f32 = self.number();
                self.add_token(TokenType::Number(number));
            }
            'a'..='z' | 'A'..='Z' | '_' => {
                let identifier: String = String::from(self.identifier());

                match keywords_map.get(&identifier[..]) {
                    Some(token_type) => self.add_token(token_type.clone()),
                    None => self.add_token(TokenType::Identifier(identifier)),
                }
            }

            _ => utils::lex_error(self.line, "unexpected character.", had_err),
        }
    }

    fn identifier(&mut self) -> &str {
        while Self::is_alnum(self.peek()) {
            self.advance();
        }

        return &self.source[self.start..self.current];
    }

    fn is_alpha(c: char) -> bool {
        return match c {
            'a'..='z' | 'A'..='Z' | '_' => true,
            _ => false,
        };
    }

    fn is_alnum(c: char) -> bool {
        return Self::is_alpha(c) || Self::is_digit(c);
    }

    fn number(&mut self) -> f32 {
        while Self::is_digit(self.peek()) {
            self.advance();
        }
        // Look for decimal part.
        if self.peek() == '.' && Self::is_digit(self.peek_next()) {
            // Consume the ".".
            self.advance();

            while Self::is_digit(self.peek()) {
                self.advance();
            }
        }

        return (&self.source[self.start..self.current])
            .parse::<f32>()
            .unwrap();
    }

    fn is_digit(c: char) -> bool {
        return match c {
            '0'..='9' => true,
            _ => false,
        };
    }

    fn string(&mut self) -> Result<String, &'static str> {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            return Err("Unterminated string.");
        }

        // Consuming the closing ".
        self.advance();

        return Ok(String::from(&self.source[self.start + 1..self.current - 1]));
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }
        return self.source.as_bytes()[self.current + 1] as char;
    }
    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        return self.source.as_bytes()[self.current] as char;
    }

    fn expected_match(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.source.as_bytes()[self.current] as char != expected {
            return false;
        }

        self.current += 1;
        return true;
    }

    fn advance(&mut self) -> char {
        let c: char = self.source.as_bytes()[self.current] as char;
        self.current += 1;
        return c;
    }

    fn add_token(&mut self, token_type: TokenType) {
        let new_token: Token = Token::new(
            token_type,
            String::from(&self.source[self.start..self.current]),
            self.line,
        );
        self.tokens.push(new_token);
    }

    fn is_at_end(&self) -> bool {
        return self.current >= self.source.len();
    }
}
