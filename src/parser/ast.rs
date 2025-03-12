use crate::lexer::Token;

macro_rules! ast {
    (pub enum $base_name:ident {$($type:ident : $container:ident $fields:tt),* $(,)?}) => {
        #[derive(Debug, Clone)]
        pub enum $base_name {
            $($type($type)),*
        }

        $(#[derive(Debug, Clone)]
        pub $container $type $fields)*
        impl $base_name {
            pub fn accept<T>(&self, visitor: &dyn Visitor<$base_name, T>) -> T {
                return visitor.visit(self);
            }
        }
    };

}
ast!(
pub enum Stmt {
    Expression : struct {
        pub expression: Box<Expr>,
    },
    Print : struct {
        pub expression: Box<Expr>,
    },
}
    );

ast!(
pub enum Expr {
    Literal : enum {
        Number(f32),
        String(String),
        True,
        False,
        Nil
    },
    Grouping : struct {
        pub expression: Box<Expr>,
    },
    Unary : struct {
        pub operator: Token,
        pub right: Box<Expr>,
    },
    Binary : struct {
        pub left: Box<Expr>,
        pub right: Box<Expr>,
        pub operator: Token,
    },
}
    );

// to see macro expantion: rustc -Zunpretty=expanded src/parser/ast.rs , should use nightly

pub trait Visitor<Production, T> {
    fn visit(&self, production: &Production) -> T;
}
