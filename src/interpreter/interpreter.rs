use crate::lexer::{Token, TokenType};
use crate::parser::{ast, Expr, Visitor};

pub enum Value {
    Number(f32),
    String(String),
    Boolean(bool),
    Nil,
}
impl Value {
    pub fn is_truthy(&self) -> bool {
        match self {
            Self::Boolean(boolean) => *boolean,
            Self::Nil => false,
            _ => true,
        }
    }
}

pub struct Interpreter;

impl Visitor<Expr, Value> for Interpreter {
    fn visit(&self, production: &Expr) -> Value {
        match production {
            Expr::Literal(literal) => match literal {
                ast::Literal::Number(number) => Value::Number(*number),
                ast::Literal::String(string) => Value::String(string.to_string()),
                ast::Literal::True => Value::Boolean(true),
                ast::Literal::False => Value::Boolean(false),
                ast::Literal::Nil => Value::Nil,
            },

            Expr::Grouping(grouping) => self.evaluate(*grouping.expression.clone()),

            Expr::Unary(unary) => {
                let right: Value = self.evaluate(*unary.right.clone());

                match unary.operator.get_token_type() {
                    TokenType::Minus => match right {
                        Value::Number(number) => Value::Number(-number),
                        _ => todo!(),
                    },
                    TokenType::Bang => Value::Boolean(!right.is_truthy()),
                    _ => todo!(),
                }
            }

            Expr::Binary(binary) => {
                let left: Value = self.evaluate(*binary.left.clone());
                let right: Value = self.evaluate(*binary.right.clone());
                match (left, right) {
                    (Value::Number(l), Value::Number(r)) => {
                        match binary.operator.get_token_type() {
                            TokenType::Minus => Value::Number(l - r),
                            TokenType::Slash => Value::Number(l / r),
                            TokenType::Star => Value::Number(l * r),
                            TokenType::Plus => Value::Number(l + r),
                            _ => todo!(),
                        }
                    },
                    (Value::String(l), Value::String(r)) => {
                        if *binary.operator.get_token_type() == TokenType::Plus {
                            let mut concated_str: String = l.clone();
                            concated_str.push_str(&r);

                            Value::String(concated_str)
                        } else {
                            todo!()
                        }
                    },
                    _ => todo!(),
                }
            }
            _ => todo!(),
        }
    }
}

//pub enum Expr {
//    Literal : enum {
//        Number(f32),
//        String(String),
//        True,
//        False,
//        Nil
//    },
//    Grouping : struct {
//        pub expression: Box<Expr>,
//    },
//    Unary : struct {
//        pub operator: Token,
//        pub right: Box<Expr>,
//    },
//    Binary : struct {
//        pub left: Box<Expr>,
//        pub right: Box<Expr>,
//        pub operator: Token,
//    },
//}

impl Interpreter {
    fn evaluate(&self, expr: Expr) -> Value {
        return expr.accept::<Value>(self);
    }
}
