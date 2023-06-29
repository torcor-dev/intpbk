use crate::lexer::Token;

#[derive(Debug)]
pub enum Statement {
    Let(Token, Token, Option<Expression>),
    Return(Token, Option<Expression>),
}

#[derive(Debug)]
pub enum Expression {
    Identifier(Token, String)
}

#[derive(Debug)]
pub enum Node {
    Program(Vec<Statement>),
}
