use crate::{lexer::{Lexer, Token}, ast::{Node, Statement}};
use anyhow::Result;

pub struct Parser {
    lexer: Lexer,
    cur_token: Option<Token>,
    peek_token: Option<Token>,
    errors: Vec<String>,
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
            _ => None
        }
    }

    fn parse_let_stmt(&mut self) -> Option<Statement> {
        let let_token = self.cur_token.take().unwrap();

        if !matches!(self.peek_token, Some(Token::Ident(_))) {
            self.peek_error(Token::Ident("identifier".to_string()));
            return None
        }

        self.next_token();

        let ident_token = self.cur_token.take().unwrap();

        if !matches!(self.peek_token, Some(Token::Assign)) {
            self.peek_error(Token::Assign);
            return None
        }

        while self.cur_token != Some(Token::Semicolon) {
            self.next_token();
        }

        Some(Statement::Let(let_token, ident_token, None))
    }

    fn peek_error(&mut self, expected: Token) {
        let msg = format!("expected next token to be {:?}, got {:?} instead", expected, self.peek_token.as_ref().unwrap());
        self.errors.push(String::from(msg))
    }

    fn parse_return_stmt(&mut self) -> Option<Statement> {
        let return_token = self.cur_token.take().unwrap();
        self.next_token();

        while self.cur_token != Some(Token::Semicolon) {
            self.next_token();
        }

        return Some(Statement::Return(return_token, None))

    }

}


#[cfg(test)]
mod tests {
    use crate::{ast::{Node, Statement, Expression}, lexer::Token};

    use super::{Lexer, Parser};
    use anyhow::Result;

    #[test]
    fn test_let_stmt() -> Result<()> {
        let input = "
            let x = 5;
            let y = 10;
            let foobar = 838383;
        ".to_string();

        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program().unwrap();

        for err in parser.errors {
           panic!("{:?}", err) 
        }

        let stmts = match program {
            Node::Program(stmts) => stmts,
            _ => panic!("Unexpected node")
        };

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
                _ => panic!("unexpected statement {:?}", stmt)
            }

        }

        Ok(())
    }

    #[test]
    fn test_return_stmt() -> Result<()> {
        let input = "
            return 5;
            return 10;
            return 838383;
        ".to_string();

        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program().unwrap();

        for err in parser.errors {
           panic!("{:?}", err) 
        }

        let stmts = match program {
            Node::Program(stmts) => stmts,
            _ => panic!("Unexpected node")
        };

        assert_eq!(stmts.len(), 3);

        for stmt in stmts {
            match stmt {
                Statement::Return(token, expr) => {
                    assert_eq!(token, Token::Return);
                    assert!(expr.is_none());
                },
                _ => panic!("unexpected statement {:?}", stmt)
            }
        }

        Ok(())
    }

    fn test_let(token: &Token, ident: &Token, _expr: &Option<Expression>, tt: &Token) {
        assert!(matches!(token, Token::Let), "Expected Let, got {:?}", token);
        assert_eq!(ident, tt)
    }
}
