mod lexer;

fn main() {
    let mut lex = lexer::Lexer::new(String::from("Hello, world"));
    let _token = lex.next_token();
}
