use crate::lexer::{Token, TokenType};
use crate::parser::{ast, Expr, Stmt, Visitor};
use crate::utils;

pub enum RuntimeError {
    // token , message
    TypeError(Token, String),
}

#[derive(Debug, Clone, PartialEq)]
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

pub struct Interpreter<'a> {
    had_runtime_err: &'a mut bool,
}

impl<'a> Interpreter<'a> {
    pub fn new(had_runtime_err: &'a mut bool) -> Interpreter<'a> {
        return Interpreter { had_runtime_err };
    }
    pub fn interpret(&mut self, statements: Vec<Stmt>) {
        for statement in statements {
            match self.execute(&statement) {
                Ok(_) => (),
                Err(err) => match err {
                    RuntimeError::TypeError(token, msg) => {
                        utils::runtime_error(&token, &msg, self.had_runtime_err)
                    }
                },
            }
        }
    }

    fn execute(&self, statement: &Stmt) -> Result<(), RuntimeError> {
        return statement.accept::<Result<(), RuntimeError>>(self);
    }

    fn evaluate(&self, expr: Expr) -> Result<Value, RuntimeError> {
        return expr.accept::<Result<Value, RuntimeError>>(self);
    }

    fn is_equal(l: Value, r: Value) -> bool {
        return l == r;
    }

    fn error<T>(token: Token, msg: String) -> Result<T, RuntimeError> {
        return Err(RuntimeError::TypeError(token, msg));
    }
}

impl<'a> Visitor<Expr, Result<Value, RuntimeError>> for Interpreter<'a> {
    fn visit(&self, production: &Expr) -> Result<Value, RuntimeError> {
        let value: Value = match production {
            Expr::Literal(literal) => match literal {
                ast::Literal::Number(number) => Value::Number(*number),
                ast::Literal::String(string) => Value::String(string.to_string()),
                ast::Literal::True => Value::Boolean(true),
                ast::Literal::False => Value::Boolean(false),
                ast::Literal::Nil => Value::Nil,
            },

            Expr::Grouping(grouping) => self.evaluate(*grouping.expression.clone())?,

            Expr::Unary(unary) => {
                let right: Value = self.evaluate(*unary.right.clone())?;

                match unary.operator.get_token_type() {
                    TokenType::Minus => match right {
                        Value::Number(number) => Value::Number(-number),
                        _ => {
                            return Self::error::<Value>(
                                unary.operator.clone(),
                                String::from("Operand must be a number."),
                            )
                        }
                    },
                    TokenType::Bang => Value::Boolean(!right.is_truthy()),
                    _ => {
                        return Self::error::<Value>(
                            unary.operator.clone(),
                            String::from("Operator is not unary."),
                        )
                    }
                }
            }

            Expr::Binary(binary) => {
                let left: Value = self.evaluate(*binary.left.clone())?;
                let right: Value = self.evaluate(*binary.right.clone())?;

                match (left.clone(), right.clone()) {
                    (Value::Number(l), Value::Number(r)) => {
                        match binary.operator.get_token_type() {
                            TokenType::Minus => Value::Number(l - r),
                            TokenType::Slash => Value::Number(l / r),
                            TokenType::Star => Value::Number(l * r),
                            TokenType::Plus => Value::Number(l + r),

                            TokenType::Greater => Value::Boolean(l > r),
                            TokenType::GreaterEqual => Value::Boolean(l >= r),
                            TokenType::Less => Value::Boolean(l < r),
                            TokenType::LessEqual => Value::Boolean(l <= r),

                            TokenType::BangEqual => Value::Boolean(!Self::is_equal(left, right)),
                            TokenType::EqualEqual => Value::Boolean(Self::is_equal(left, right)),

                            _ => {
                                return Self::error::<Value>(
                                    binary.operator.clone(),
                                    String::from("Operator cannot be applied on two numbers"),
                                )
                            }
                        }
                    }
                    (Value::String(l), Value::String(r)) => {
                        match binary.operator.get_token_type() {
                            TokenType::Plus => {
                                let mut concated_str: String = l.clone();
                                concated_str.push_str(&r);

                                Value::String(concated_str)
                            }
                            TokenType::BangEqual => Value::Boolean(!Self::is_equal(left, right)),
                            TokenType::EqualEqual => Value::Boolean(Self::is_equal(left, right)),
                            _ => {
                                return Self::error::<Value>(
                                    binary.operator.clone(),
                                    String::from("Operator cannot be applied on two strings"),
                                )
                            }
                        }
                    }
                    _ => match binary.operator.get_token_type() {
                        TokenType::BangEqual => Value::Boolean(!Self::is_equal(left, right)),
                        TokenType::EqualEqual => Value::Boolean(Self::is_equal(left, right)),
                        TokenType::Plus => {
                            return Self::error::<Value>(
                                binary.operator.clone(),
                                String::from("Operands must be two number or two strings."),
                            )
                        }
                        _ => {
                            return Self::error::<Value>(
                                binary.operator.clone(),
                                String::from("Operands must be a number."),
                            )
                        }
                    },
                }
            }
        };

        return Ok(value);
    }
}

impl<'a> Visitor<Stmt, Result<(), RuntimeError>> for Interpreter<'a> {
    fn visit(&self, statement: &Stmt) -> Result<(), RuntimeError> {
        match statement {
            Stmt::Expression(expression) => {
                self.evaluate(*expression.expression.clone())?;
            }
            Stmt::Print(print) => {
                let value: Value = self.evaluate(*print.expression.clone())?;
                println!("{:?}", value);
            }
        };
        return Ok(());
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
