mod expression;
mod parser;
mod scanner;
mod token;
use parser::Parser;
use scanner::Scanner;
use std::error::Error;

pub fn run(source_code: &str) -> Result<(), Box<dyn Error>> {
    // run through source_code, character by character, adding tokens as we go
    let mut scanner = Scanner::new(source_code);

    let tokens = scanner.scan()?;

    let parser = Parser::new(&tokens);

    let ast = parser.parse();

    Ok(())
}
