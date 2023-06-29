use crate::lexer::Token;

pub enum Statement {
    Let(Token, Token, Option<Expression>) 
}

pub enum Expression {
    Identifier(Token, String)
}

pub enum Node {
    Program(Vec<Statement>),
}
