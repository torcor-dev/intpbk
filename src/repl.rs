use std::io::{self, BufRead, Write};
use anyhow::Result;

use crate::{lexer::{self, Token}, parser};

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
        let parser = parser::Parser::new(l);


        print!("{}", PROMPT);
        stdout_lock.flush()?;

    }

    return Ok(())
}
