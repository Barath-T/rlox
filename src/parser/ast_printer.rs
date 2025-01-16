use super::ast::{Expr, Literal, Visitor};

pub struct AstPrinter;

impl AstPrinter {
    pub fn print(&self, expr: &Expr) -> String {
        return expr.accept::<String>(self);
    }
    fn parenthesize(&self, name: String, exprs: Vec<&Expr>) -> String {
        let mut result: String = String::new();

        result.push('(');
        result.push_str(&name);
        for expr in exprs {
            result.push(' ');
            result.push_str(&expr.accept::<String>(self));
        }
        result.push(')');

        return result;
    }
}

impl Visitor<Expr, String> for AstPrinter {
    fn visit(&self, expr: &Expr) -> String {
        match expr {
            Expr::Literal(literal) => match literal {
                Literal::Nil => String::from("nil"),
                Literal::True => String::from("true"),
                Literal::False => String::from("false"),
                Literal::String(string) => string.clone(),
                Literal::Number(number) => number.to_string(),
            },
            Expr::Binary(binary) => self.parenthesize(
                binary.operator.get_lexeme(),
                vec![&binary.left, &binary.right],
            ),
            Expr::Grouping(grouping) => {
                self.parenthesize(String::from("group"), vec![&grouping.expression])
            }
            Expr::Unary(unary) => {
                self.parenthesize(unary.operator.get_lexeme(), vec![&unary.right])
            }
        }
    }
}

// implement test
// let expr: ast::Expr = ast::Expr::Binary(ast::Binary {
//     left: Box::new(ast::Expr::Unary(ast::Unary {
//         operator: Token::new(lexer::token::TokenType::Minus, String::from("-"), 1),
//         right: Box::new(ast::Expr::Literal(ast::Literal::Number(123.0))),
//     })),
//     operator: Token::new(lexer::token::TokenType::Star, String::from("*"), 1),
//     right: Box::new(ast::Expr::Grouping(ast::Grouping {
//         expression: Box::new(ast::Expr::Literal(ast::Literal::Number(45.67))),
//     })),
// });
// println!("{}", ast_printer::AstPrinter.print(&expr));
//
// return;
