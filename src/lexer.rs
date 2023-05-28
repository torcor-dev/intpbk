#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    Illegal,
    Eof,
    Ident(String),
    Int(String),
    Assign,
    Plus,
    Comma,
    Semicolon,
    Lparen,
    Rparen,
    Lbrace,
    Rbrace,
    Function,
    Let,
}

pub struct Lexer {
    input: String,
    pos: usize,
    read_pos: usize,
    ch: u8,
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        let mut l = Lexer {
            input,
            pos: 0,
            read_pos: 0,
            ch: 0,
        };
        l.read_char();
        return l;
    }

    fn read_char(&mut self) {
        if self.read_pos >= self.input.len() {
            self.ch = 0
        } else {
            self.ch = self.input.as_bytes()[self.read_pos]
        }
        self.pos = self.read_pos;
        self.read_pos += 1
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let token = match self.ch {
            b'=' => Token::Assign,
            b';' => Token::Semicolon,
            b'(' => Token::Lparen,
            b')' => Token::Rparen,
            b',' => Token::Comma,
            b'+' => Token::Plus,
            b'{' => Token::Lbrace,
            b'}' => Token::Rbrace,

            0 => Token::Eof,
            _ => {
                if is_letter(self.ch) {
                    return lookup_ident(self.read_ident());
                } else if is_digit(self.ch) {
                    return Token::Int(self.read_number());
                } else {
                    Token::Illegal
                }
            }
        };

        self.read_char();

        return token;
    }

    fn read_ident(&mut self) -> String {
        let pos = self.pos;
        while is_letter(self.ch) {
            self.read_char()
        }
        return self.input[pos..self.pos].to_string();
    }

    fn read_number(&mut self) -> String {
        let pos = self.pos;
        while is_digit(self.ch) {
            self.read_char()
        }
        return self.input[pos..self.pos].to_string();
    }

    fn skip_whitespace(&mut self) {
        while self.ch == b' ' || self.ch == b'\t' || self.ch == b'\n' || self.ch == b'\r' {
            self.read_char()
        }
    }
}

fn lookup_ident(ident: String) -> Token {
    match &*ident {
        "fn" => Token::Function,
        "let" => Token::Let,
        _ => Token::Ident(ident),
    }
}

fn is_digit(ch: u8) -> bool {
    return b'0' <= ch && ch <= b'9';
}

fn is_letter(ch: u8) -> bool {
    return b'a' <= ch && ch <= b'z' || b'A' <= ch && ch <= b'Z' || ch == b'_';
}

#[cfg(test)]
mod tests {
    use super::{Lexer, Token};

    #[test]
    fn test_next_token() {
        let input = String::from(
            "
            let five = 5;
            let ten = 10;
            let add = fn(x, y) {
                x + y;
            };
            let result = add(five, ten);",
        );
        let tests = [
            Token::Let,
            Token::Ident(String::from("five")),
            Token::Assign,
            Token::Int(String::from("5")),
            Token::Semicolon,
            Token::Let,
            Token::Ident(String::from("ten")),
            Token::Assign,
            Token::Int(String::from("10")),
            Token::Semicolon,
            Token::Let,
            Token::Ident(String::from("add")),
            Token::Assign,
            Token::Function,
            Token::Lparen,
            Token::Ident(String::from("x")),
            Token::Comma,
            Token::Ident(String::from("y")),
            Token::Rparen,
            Token::Lbrace,
            Token::Ident(String::from("x")),
            Token::Plus,
            Token::Ident(String::from("y")),
            Token::Semicolon,
            Token::Rbrace,
            Token::Semicolon,
            Token::Let,
            Token::Ident(String::from("result")),
            Token::Assign,
            Token::Ident(String::from("add")),
            Token::Lparen,
            Token::Ident(String::from("five")),
            Token::Comma,
            Token::Ident(String::from("ten")),
            Token::Rparen,
            Token::Semicolon,
            Token::Eof,
        ];

        let mut l = Lexer::new(input);

        for tt in tests {
            let tok = l.next_token();
            assert_eq!(tok, tt);
        }
    }
}
