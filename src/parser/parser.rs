use super::{ast, ast::Expr, ast::Stmt};
use crate::lexer::{Token, TokenType};
use crate::utils;

pub struct Parser<'a> {
    tokens: Vec<Token>,
    current: usize,
    had_err: &'a mut bool,
}
enum ParseError {
    Bad { msg: String, token: Token },
}

impl<'a> Parser<'a> {
    pub fn new(tokens: Vec<Token>, had_err: &'a mut bool) -> Parser<'a> {
        return Self {
            tokens,
            current: 0,
            had_err,
        };
    }

    pub fn parse(&mut self) -> Vec<Stmt> {
        let mut statements: Vec<Stmt> = Vec::new();

        while !self.is_at_end() {
            match self.statement() {
                Ok(stmt) => statements.push(stmt),
                Err(_) => (),
            }
        }
        return statements;
    }


    // statement -> expr_stmt | print_stmt ;
    fn statement(&mut self) -> Result<Stmt, ParseError> {
        let match_print: Vec<TokenType> = vec![TokenType::Print];

        if self.match_token_type(&match_print) {
            return self.print_statement();
        }
        return self.expression_statement();
    }

    // print_stmt -> "print" expression ";" ;
    fn print_statement(&mut self) -> Result<Stmt, ParseError> {
        let value: Expr = self.expression()?;
        let _ = self.consume(
            TokenType::Semicolon,
            String::from("Expect ';' after value."),
        );
        return Ok(Stmt::Print(ast::Print {
            expression: Box::new(value),
        }));
    }

    // expr_stmt -> expression ";" ;
    fn expression_statement(&mut self) -> Result<Stmt, ParseError> {
        let expr: Expr = self.expression()?;
        let _ = self.consume(
            TokenType::Semicolon,
            String::from("Expect ';' after expression."),
        );
        return Ok(Stmt::Expression(ast::Expression {
            expression: Box::new(expr),
        }));
    }

    //expression -> equality ;
    fn expression(&mut self) -> Result<Expr, ParseError> {
        return self.equality();
    }

    //equality -> comparison (("!=" | "==") comparison)* ;
    fn equality(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.comparison()?;

        let types_to_match: Vec<TokenType> = vec![TokenType::BangEqual, TokenType::EqualEqual];

        while self.match_token_type(&types_to_match) {
            let operator: Token = self.previous().clone();
            let right: Expr = self.comparison()?;
            expr = Expr::Binary(ast::Binary {
                left: Box::new(expr),
                operator: operator,
                right: Box::new(right),
            });
        }

        return Ok(expr);
    }

    //comparison -> term ((">" | ">=" | "<=") term)* ;
    fn comparison(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.term()?;

        let types_to_match: Vec<TokenType> = vec![
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ];

        while self.match_token_type(&types_to_match) {
            let operator: Token = self.previous().clone();
            let right: Expr = self.term()?;
            expr = Expr::Binary(ast::Binary {
                left: Box::new(expr),
                operator: operator,
                right: Box::new(right),
            });
        }

        return Ok(expr);
    }

    //term -> factor (("+" | "-") factor)* ;
    fn term(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.factor()?;

        let types_to_match: Vec<TokenType> = vec![TokenType::Plus, TokenType::Minus];

        while self.match_token_type(&types_to_match) {
            let operator: Token = self.previous().clone();
            let right: Expr = self.factor()?;
            expr = Expr::Binary(ast::Binary {
                left: Box::new(expr),
                operator: operator,
                right: Box::new(right),
            });
        }

        return Ok(expr);
    }

    //factor -> unary (("*" | "/") unary)* ;
    fn factor(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.unary()?;

        let types_to_match: Vec<TokenType> = vec![TokenType::Star, TokenType::Slash];

        while self.match_token_type(&types_to_match) {
            let operator: Token = self.previous().clone();
            let right: Expr = self.unary()?;
            expr = Expr::Binary(ast::Binary {
                left: Box::new(expr),
                operator: operator,
                right: Box::new(right),
            });
        }

        return Ok(expr);
    }

    //unary -> ("!" | "-") unary | primary ;
    fn unary(&mut self) -> Result<Expr, ParseError> {
        let types_to_match: Vec<TokenType> = vec![TokenType::Bang, TokenType::Minus];

        if self.match_token_type(&types_to_match) {
            let operator: Token = self.previous().clone();
            let right: Expr = self.unary()?;
            let expr = Expr::Unary(ast::Unary {
                operator: operator,
                right: Box::new(right),
            });
            return Ok(expr);
        }

        return self.primary();
    }

    //primary -> NUMBER | STRING | "true" | "false" | "nil" | "(" expression ")" ;
    fn primary(&mut self) -> Result<Expr, ParseError> {
        let expr = match self.advance().get_token_type() {
            TokenType::False => Expr::Literal(ast::Literal::False),
            TokenType::True => Expr::Literal(ast::Literal::True),
            TokenType::Nil => Expr::Literal(ast::Literal::Nil),
            TokenType::String(string) => Expr::Literal(ast::Literal::String(string.clone())),
            TokenType::Number(number) => Expr::Literal(ast::Literal::Number(*number)),
            TokenType::LeftParen => {
                let expr: Expr = self.expression()?;
                self.consume(
                    TokenType::RightParen,
                    String::from("Expect ')' after expression."),
                )?;
                Expr::Grouping(ast::Grouping {
                    expression: Box::new(expr),
                })
            }
            _ => self.error(self.peek().clone(), String::from("Expected expression!"))?,
        };

        return Ok(expr);
    }
    fn consume(&mut self, token_type: TokenType, message: String) -> Result<&Token, ParseError> {
        if self.check_token_type(&token_type) {
            return Ok(self.advance());
        }
        return self.error::<&Token>(self.peek().clone(), message);
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
        if self.current <= 0 {
            return &self.tokens[self.current];
        }
        return &self.tokens[self.current - 1];
    }
    fn error<T>(&mut self, token: Token, msg: String) -> Result<T, ParseError> {
        utils::parse_error(&token, &msg, self.had_err);

        // panic!("{}: {}", msg, token.to_string());

        return Err(ParseError::Bad { msg, token });
    }
    fn synchronize(&mut self) {
        self.advance();
        while !self.is_at_end() {
            if *self.previous().get_token_type() == TokenType::Semicolon {
                return;
            }

            match self.peek().get_token_type() {
                TokenType::Class
                | TokenType::Fun
                | TokenType::Var
                | TokenType::For
                | TokenType::If
                | TokenType::While
                | TokenType::Print
                | TokenType::Return => return,
                _ => todo!(),
            }
            self.advance();
        }
    }
}
