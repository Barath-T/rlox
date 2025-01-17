use super::{ast, ast::Expr};
use crate::lexer::{Token, TokenType};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        return Self { tokens, current: 0 };
    }

    //expression -> equality ;
    fn expression(&mut self) -> Expr {
        return self.equality();
    }

    //equality -> comparison (("!=" | "==") comparison)* ;
    fn equality(&mut self) -> Expr {
        let mut expr = self.comparison();

        let types_to_match: Vec<TokenType> = vec![TokenType::BangEqual, TokenType::EqualEqual];

        while self.match_token_type(&types_to_match) {
            let operator: Token = self.previous().clone();
            let right: Expr = self.comparison();
            expr = Expr::Binary(ast::Binary {
                left: Box::new(expr),
                operator: operator,
                right: Box::new(right),
            });
        }

        return expr;
    }

    //comparison -> term ((">" | ">=" | "<=") term)* ;
    fn comparison(&mut self) -> Expr {
        let mut expr = self.term();

        let types_to_match: Vec<TokenType> = vec![
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ];

        while self.match_token_type(&types_to_match) {
            let operator: Token = self.previous().clone();
            let right: Expr = self.term();
            expr = Expr::Binary(ast::Binary {
                left: Box::new(expr),
                operator: operator,
                right: Box::new(right),
            });
        }

        return expr;
    }

    //term -> factor (("+" | "-") factor)* ;
    fn term(&mut self) -> Expr {
        let mut expr = self.factor();

        let types_to_match: Vec<TokenType> = vec![TokenType::Plus, TokenType::Minus];

        while self.match_token_type(&types_to_match) {
            let operator: Token = self.previous().clone();
            let right: Expr = self.factor();
            expr = Expr::Binary(ast::Binary {
                left: Box::new(expr),
                operator: operator,
                right: Box::new(right),
            });
        }

        return expr;
    }

    //factor -> unary (("*" | "/") unary)* ;
    fn factor(&mut self) -> Expr {
        let mut expr = self.unary();

        let types_to_match: Vec<TokenType> = vec![TokenType::Star, TokenType::Slash];

        while self.match_token_type(&types_to_match) {
            let operator: Token = self.previous().clone();
            let right: Expr = self.unary();
            expr = Expr::Binary(ast::Binary {
                left: Box::new(expr),
                operator: operator,
                right: Box::new(right),
            });
        }

        return expr;
    }

    //unary -> ("!" | "-") unary | primary ;
    fn unary(&mut self) -> Expr {
        let types_to_match: Vec<TokenType> = vec![TokenType::Bang, TokenType::Minus];

        if self.match_token_type(&types_to_match) {
            let operator: Token = self.previous().clone();
            let right: Expr = self.unary();
            let expr = Expr::Unary(ast::Unary {
                operator: operator,
                right: Box::new(right),
            });
            return expr;
        }

        return self.primary();
    }

    //primary -> NUMBER | STRING | "true" | "false" | "nil" | "(" expression ")" ;
    fn primary(&mut self) -> Expr {
        match self.advance().get_token_type() {
            TokenType::False => Expr::Literal(ast::Literal::False),
            TokenType::True => Expr::Literal(ast::Literal::True),
            TokenType::Nil => Expr::Literal(ast::Literal::Nil),
            TokenType::String(string) => Expr::Literal(ast::Literal::String(string.clone())),
            TokenType::Number(number) => Expr::Literal(ast::Literal::Number(*number)),
            TokenType::LeftParen => {
                let expr: Expr = self.expression();
                self.consume(TokenType::RightParen, "Expect ')' after expression.");
                return Expr::Grouping(ast::Grouping { expression: Box::new(expr) });
            }
            _ => todo!(),
        }
    }
    fn consume(&self, token_type:TokenType, expect: &'static str){
        todo!();
    }

    fn match_token_type(&mut self, types: &Vec<TokenType>) -> bool {
        for token_type in types {
            if self.check_token_type(&token_type) {
                self.advance();
                return true;
            }
        }

        return false;
    }
    fn check_token_type(&self, token_type: &TokenType) -> bool {
        match self.is_at_end() {
            true => return false,
            false => *self.peek().get_token_type() == *token_type,
        }
    }
    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }

        return self.previous();
    }
    fn is_at_end(&self) -> bool {
        return *self.peek().get_token_type() == TokenType::Eof;
    }
    fn peek(&self) -> &Token {
        return &self.tokens[self.current];
    }
    fn previous(&self) -> &Token {
        return &self.tokens[self.current - 1];
    }
}
