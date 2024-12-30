use crate::token::Token;

pub struct Scanner {
    source: String,
    start: u32,
    current: u32,
    tokens: Vec<Token>,
}

impl Scanner {
    pub fn new(source_code: &str) -> Self {
        Self {
            start: 0,
            current: 0,
            tokens: Vec::new(),
            source: source_code.to_owned(),
        }
    }

    pub fn scan(&mut self) {
        let chars = self.source.chars();
        // let mut current_chars = Vec::new();
        let mut current_lexeme = String::new();

        for c in chars {
            current_lexeme.push(c);
            println!("current_lexeme: {}", current_lexeme);

            if let Some(token) = Token::from_string(&current_lexeme, &self.tokens) {
                println!("Found token: {:?}", token);
                self.tokens.push(token);
                current_lexeme = String::new();
            }
        }

        println!("Found tokens: {:?}", self.tokens);
    }
}
