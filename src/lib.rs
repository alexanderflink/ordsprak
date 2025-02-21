mod interpreter;
mod parser;

pub fn run(source_code: &str) -> Result<(), nom::Err<nom::error::Error<&str>>> {
    let parsed_statements = parser::parse(source_code)?;

    println!("Parsed statements: {:?}", parsed_statements);

    let mut interpreter = interpreter::Interpreter::new();
    interpreter.interpret(parsed_statements);

    Ok(())
}
