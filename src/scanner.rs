use crate::token::{Token, TokenType};
use core::fmt;

pub struct Scanner {
    source: String,
    start: usize,
    current: usize,
    tokens: Vec<Token>,
}

#[derive(Debug)]
pub enum ScannerError {
    SyntaxError,
}

impl fmt::Display for ScannerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ScannerError::SyntaxError => write!(f, "SyntaxError!"),
        }
    }
}

impl std::error::Error for ScannerError {}

impl Scanner {
    pub fn new(source_code: &str) -> Self {
        Self {
            start: 0,
            current: 0,
            tokens: Vec::new(),
            source: source_code.to_owned(),
        }
    }

    pub fn scan(&mut self) -> Result<(), ScannerError> {
        let chars = self.source.chars();
        let mut scanning_chars = String::new();

        for c in chars {
            scanning_chars.push(c);

            // println!("scanning_chars: {}", scanning_chars);

            if let Some(token) = Token::from_string(&scanning_chars.trim()) {
                scanning_chars.clear();
                self.tokens.push(token);
            }
        }

        println!("Found tokens: {:?}", self.tokens);

        Ok(())
    }
}
