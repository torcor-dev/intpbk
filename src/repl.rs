use std::io::{self, BufRead, Write};
use anyhow::Result;

use crate::lexer::{self, Token};

const PROMPT: &str = ">> ";

pub fn start() -> Result<()> {
    let stdin = io::stdin();
    let stdout = io::stdout();

    let mut lines = stdin.lock().lines();
    let mut stdout_lock = stdout.lock();


    println!("Welcome to the Monkey REPL");

    print!("{PROMPT}");
    stdout_lock.flush()?;

    while let Some(line) = lines.next() {
        let mut l = lexer::Lexer::new(line?);


        let mut tok = l.next_token()?;
        while tok != Token::Eof {
            print!("{:?} ", tok);
            tok = l.next_token()?;
        }
        println!();
        print!("{}", PROMPT);
        stdout_lock.flush()?;

    }

    return Ok(())
}
