use crate::{
    ast::{Expression, Node, Statement},
    lexer::{Lexer, Token},
};
use anyhow::Result;

pub struct Parser {
    lexer: Lexer,
    cur_token: Option<Token>,
    peek_token: Option<Token>,
    errors: Vec<String>,
}

// Precedence:
const LOWEST: usize = 1;
const EQUALS: usize = 2;
const LESSGREATER: usize = 3;
const SUM: usize = 4;
const PRODUCT: usize = 5;
const PREFIX: usize = 6;
const CALL: usize = 7;

fn precedence(token: &Option<Token>) -> usize {
    match token.as_ref().unwrap() {
        Token::Eq | Token::Neq => EQUALS,
        Token::Lt | Token::Gt => LESSGREATER,
        Token::Plus | Token::Minus => SUM,
        Token::Slash | Token::Asterisk => PRODUCT,
        Token::Lparen => CALL,
        _ => LOWEST,
    }
}

impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        let mut parser = Parser {
            lexer,
            cur_token: None,
            peek_token: None,
            errors: vec![],
        };

        parser.next_token();
        parser.next_token();

        return parser;
    }

    pub fn parse_program(&mut self) -> Result<Node> {
        let mut statements: Vec<Statement> = Vec::new();

        while self.cur_token != Some(Token::Eof) {
            if let Some(stmt) = self.parse_stmt() {
                statements.push(stmt)
            }
            self.next_token();
        }

        Ok(Node::Program(statements))
    }

    fn next_token(&mut self) {
        self.cur_token = self.peek_token.take();
        self.peek_token = Some(self.lexer.next_token().expect("Where's my token?"));
    }

    fn parse_stmt(&mut self) -> Option<Statement> {
        match self.cur_token {
            Some(Token::Let) => self.parse_let_stmt(),
            Some(Token::Return) => self.parse_return_stmt(),
            _ => self.parse_expr_stmt(),
        }
    }

    fn parse_let_stmt(&mut self) -> Option<Statement> {
        let let_token = self.cur_token.take().unwrap();

        if !matches!(self.peek_token, Some(Token::Ident(_))) {
            self.peek_error(Token::Ident("identifier".to_string()));
            return None;
        }

        self.next_token();

        let ident_token = self.cur_token.take().unwrap();

        if !matches!(self.peek_token, Some(Token::Assign)) {
            self.peek_error(Token::Assign);
            return None;
        }

        while self.cur_token != Some(Token::Semicolon) {
            self.next_token();
        }

        Some(Statement::Let(let_token, ident_token, None))
    }

    fn peek_error(&mut self, expected: Token) {
        let msg = format!(
            "expected next token to be {:?}, got {:?} instead",
            expected,
            self.peek_token.as_ref().unwrap()
        );
        self.errors.push(String::from(msg))
    }

    fn parse_return_stmt(&mut self) -> Option<Statement> {
        let return_token = self.cur_token.take().unwrap();
        self.next_token();

        while self.cur_token != Some(Token::Semicolon) {
            self.next_token();
        }

        return Some(Statement::Return(return_token, None));
    }

    fn parse_expr_stmt(&mut self) -> Option<Statement> {
        let tok = self.cur_token.clone().unwrap();

        let expr = self.parse_expr(LOWEST);

        if self.peek_token == Some(Token::Semicolon) {
            self.next_token();
        }

        return Some(Statement::Expression(tok, expr));
    }

    fn parse_expr(&mut self, prec: usize) -> Option<Box<Expression>> {
        if !is_prefix_op(self.cur_token.as_ref().unwrap()) {
            self.errors.push(format!(
                "no prefix parse function for {}",
                self.cur_token.as_ref().unwrap()
            ));
            return None;
        }

        let mut left = self.parse_prefix();

        while self.peek_token != Some(Token::Semicolon) && prec < precedence(&self.peek_token) {
            if !is_infix_op(self.peek_token.as_ref().unwrap()) {
                return left;
            }

            self.next_token();

            left = self.parse_infix(left);
        }

        return left;
    }

    fn parse_identifier(&self) -> Option<Box<Expression>> {
        Some(Box::new(Expression::Identifier(
            self.cur_token.clone().unwrap(),
        )))
    }

    fn parse_integer_literal(&self) -> Option<Box<Expression>> {
        let token = self.cur_token.clone();
        if let Token::Int(val) = token.as_ref().unwrap() {
            let lit: i64 = val.parse().unwrap();

            Some(Box::new(Expression::IntegerLiteral(token.unwrap(), lit)))
        } else {
            None
        }
    }

    fn parse_prefix(&mut self) -> Option<Box<Expression>> {
        match self.cur_token.as_ref() {
            Some(Token::Ident(_)) => self.parse_identifier(),
            Some(Token::Int(_)) => self.parse_integer_literal(),
            Some(Token::Bang) | Some(Token::Minus) => self.parse_prefix_expr(),
            _ => None,
        }
    }

    fn parse_infix(&mut self, left: Option<Box<Expression>>) -> Option<Box<Expression>> {
        match self.cur_token.as_ref() {
            Some(Token::Plus) => self.parse_infix_expr(left),
            Some(Token::Minus) => self.parse_infix_expr(left),
            Some(Token::Slash) => self.parse_infix_expr(left),
            Some(Token::Asterisk) => self.parse_infix_expr(left),
            Some(Token::Eq) => self.parse_infix_expr(left),
            Some(Token::Neq) => self.parse_infix_expr(left),
            Some(Token::Lt) => self.parse_infix_expr(left),
            Some(Token::Gt) => self.parse_infix_expr(left),
            _ => None,
        }
    }

    fn parse_prefix_expr(&mut self) -> Option<Box<Expression>> {
        let token = self.cur_token.take();

        self.next_token();

        let right = self.parse_expr(PREFIX);

        Some(Box::new(Expression::Prefix(token.unwrap(), right)))
    }


    fn parse_infix_expr(&mut self, left: Option<Box<Expression>>) -> Option<Box<Expression>> {
        let operator = self.cur_token.take();
        let precedence = precedence(&operator);

        self.next_token();
        let right = self.parse_expr(precedence);

        Some(Box::new(Expression::Infix(left, operator.unwrap(), right)))
    }
}

fn is_prefix_op(token: &Token) -> bool {
    match token {
        Token::Ident(_) => true,
        Token::Int(_) => true,
        Token::Bang | Token::Minus => true,
        _ => false,
    }
}



fn is_infix_op(token: &Token) -> bool {
    match token {
        Token::Plus => true,
        Token::Minus => true,
        Token::Slash => true,
        Token::Asterisk => true,
        Token::Eq => true,
        Token::Neq => true,
        Token::Lt => true,
        Token::Gt => true,
        _ => false,
    }
}


#[cfg(test)]
mod tests {
    use crate::{
        ast::{Expression, Node, Statement},
        lexer::Token,
    };

    use super::{Lexer, Parser};
    use anyhow::{Ok, Result};

    #[test]
    fn test_let_stmt() -> Result<()> {
        let stmts = create_program(
            "let x = 5;
            let y = 10;
            let foobar = 838383;",
        );

        assert_eq!(stmts.len(), 3);

        let tests = [
            Token::Ident(String::from("x")),
            Token::Ident(String::from("y")),
            Token::Ident(String::from("foobar")),
        ];

        for (i, tt) in tests.iter().enumerate() {
            let stmt = &stmts[i];
            match stmt {
                Statement::Let(token, ident, expr) => test_let(token, ident, expr, tt),
                _ => panic!("unexpected statement {:?}", stmt),
            }
        }

        Ok(())
    }

    #[test]
    fn test_return_stmt() -> Result<()> {
        let stmts = create_program(
            "return 5;
            return 10;
            return 838383;",
        );

        assert_eq!(stmts.len(), 3);

        for stmt in stmts {
            match stmt {
                Statement::Return(token, expr) => {
                    assert_eq!(token, Token::Return);
                    assert!(expr.is_none());
                }
                _ => panic!("unexpected statement {:?}", stmt),
            }
        }

        Ok(())
    }

    fn test_let(token: &Token, ident: &Token, _expr: &Option<Box<Expression>>, tt: &Token) {
        assert!(matches!(token, Token::Let), "Expected Let, got {:?}", token);
        assert_eq!(ident, tt)
    }

    #[test]
    fn test_identifier_expr() -> Result<()> {
        let stmts = create_program("foobar;");
        assert_eq!(stmts.len(), 1);

        for stmt in stmts {
            match stmt {
                Statement::Expression(_, expr) => match **expr.as_ref().unwrap() {
                    Expression::Identifier(ref value) => {
                        assert_eq!(*value, Token::Ident("foobar".to_string()));
                    }
                    _ => panic!("unexpected expression {:?}", expr),
                },
                _ => panic!("unexpected statement {:?}", stmt),
            }
        }

        Ok(())
    }

    #[test]
    fn test_int_literal_expr() -> Result<()> {
        let stmts = create_program("42;");
        assert_eq!(stmts.len(), 1);

        for stmt in stmts {
            match stmt {
                Statement::Expression(_, expr) => {
                    if let Expression::IntegerLiteral(token, value) = &**expr.as_ref().unwrap() {
                        assert_eq!(*token, Token::Int("42".to_string()));
                        assert_eq!(*value, 42);
                    } else {
                        panic!("unexpected expression {:?}", expr);
                    }
                }
                _ => panic!("unexpected statement {:?}", stmt),
            }
        }

        Ok(())
    }

    #[test]
    fn test_prefix_expr() -> Result<()> {
        let stmt = create_program("!5;");

        assert_eq!(stmt.len(), 1);

        match stmt.first().unwrap() {
            Statement::Expression(_, expr) => match **expr.as_ref().unwrap() {
                Expression::Prefix(ref token, ref expr) => {
                    assert_eq!(*token, Token::Bang);
                    assert_eq!(5, expr_to_int(expr.as_ref().unwrap()))
                }
                _ => panic!("unexpected expression {:?}", expr),
            },
            _ => panic!("unexpected statement {:?}", stmt),
        }

        Ok(())
    }

    #[test]
    fn test_infix_expr() -> Result<()> {
        struct TC<'a> {
            input: &'a str,
            left: i64,
            operator: Token,
            right: i64,
        }

        let cases = vec![
            TC {
                input: "5 + 5;",
                left: 5,
                operator: Token::Plus,
                right: 5,
            },
            TC {
                input: "5 - 5;",
                left: 5,
                operator: Token::Minus,
                right: 5,
            },
            TC {
                input: "5 * 5;",
                left: 5,
                operator: Token::Asterisk,
                right: 5,
            },
            TC {
                input: "5 / 5;",
                left: 5,
                operator: Token::Slash,
                right: 5,
            },
            TC {
                input: "5 > 5;",
                left: 5,
                operator: Token::Gt,
                right: 5,
            },
            TC {
                input: "5 < 5;",
                left: 5,
                operator: Token::Lt,
                right: 5,
            },
            TC {
                input: "5 == 5;",
                left: 5,
                operator: Token::Eq,
                right: 5,
            },
            TC {
                input: "5 != 5;",
                left: 5,
                operator: Token::Neq,
                right: 5,
            },
        ];

        for tc in cases {
            let stmt = create_program(tc.input);

            match stmt.first().unwrap() {
                Statement::Expression(_, expr) => match **expr.as_ref().unwrap() {
                    Expression::Infix(ref left, ref op, ref right) => {
                        assert_eq!(tc.left, expr_to_int(left.as_ref().unwrap()));
                        assert_eq!(*op, tc.operator);
                        assert_eq!(tc.right, expr_to_int(right.as_ref().unwrap()));
                    }
                    _ => panic!("unexpected expression {:?}", expr),
                },
                _ => panic!("unexpected statement {:?}", stmt),
            }
        }

        Ok(())
    }

    #[test]
    fn test_operator_precedence() -> Result<()> {
        let tests = vec![
            ("-a * b", "((-a) * b)"),
            ("!-a", "(!(-a))"),
            ("a + b + c", "((a + b) + c)"),
            ("a + b - c", "((a + b) - c)"),
            ("a * b * c", "((a * b) * c)"),
            ("a * b / c", "((a * b) / c)"),
            ("a + b / c", "(a + (b / c))"),
            ("a + b * c + d / e - f", "(((a + (b * c)) + (d / e)) - f)"),
            ("3 + 4; -5 * 5", "(3 + 4)((-5) * 5)"),
            ("5 > 4 == 3 < 4", "((5 > 4) == (3 < 4))"),
            ("5 < 4 != 3 > 4", "((5 < 4) != (3 > 4))"),
            (
                "3 + 4 * 5 == 3 * 1 + 4 * 5",
                "((3 + (4 * 5)) == ((3 * 1) + (4 * 5)))",
            ),
            (
                "3 + 4 * 5 == 3 * 1 + 4 * 5",
                "((3 + (4 * 5)) == ((3 * 1) + (4 * 5)))",
            ),
        ];

        for t in tests {
            let stmts = create_program(t.0);

            let str = stmts.iter().map(|s| s.to_string()).collect::<Vec<_>>().join("");

            assert_eq!(str, t.1);
        }

        Ok(())
    }

    fn expr_to_int(expr: &Box<Expression>) -> i64 {
        match **expr {
            Expression::IntegerLiteral(_, val) => val,
            _ => panic!(
                "unexpected expression, expected IntegerLiteral, got {:?}",
                expr
            ),
        }
    }

    fn create_program(input: &str) -> Vec<Statement> {
        let lexer = Lexer::new(input.to_string());
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program().unwrap();

        for err in parser.errors {
            panic!("{:?}", err)
        }

        let stmts = match program {
            Node::Program(stmts) => stmts,
        };

        return stmts;
    }
}
