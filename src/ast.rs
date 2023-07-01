use std::fmt::Display;

use crate::lexer::Token;

#[derive(Debug)]
pub enum Statement {
    Let(Token, Token, Option<Expression>),
    Return(Token, Option<Expression>),
    Expression(Token, Option<Expression>),
}

#[derive(Debug)]
pub enum Expression {
    Identifier(Token),
}

#[derive(Debug)]
pub enum Node {
    Program(Vec<Statement>),
}

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expression::Identifier(token) => write!(f, "{}", token)?,
        }
        Ok(())
    }
}

impl Display for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Statement::Let(keyword, identifier, expression) => {
                write!(f, "{} {} = ", keyword, identifier)?;
                if let Some(expr) = expression {
                    write!(f, "{}", expr)?;
                }
                Ok(())
            }
            Statement::Return(keyword, expression) => {
                write!(f, "{}", keyword)?;
                if let Some(expr) = expression {
                    write!(f, " {}", expr)?;
                }
                Ok(())
            }
            Statement::Expression(_, expression) => {
                if let Some(expr) = expression {
                    write!(f, "{}", expr)?;
                }
                Ok(())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use anyhow::{Ok, Result};

    use crate::lexer::Token;

    use super::{Expression, Statement};

    #[test]
    fn print_program() -> Result<()> {
        let stmts = [Statement::Let(
            Token::Let,
            Token::Ident(String::from("foo")),
            Some(Expression::Identifier(Token::Ident(String::from("bar")))),
        )];

        for stmt in stmts {
            println!("{}", stmt);
        }

        Ok(())
    }
}
