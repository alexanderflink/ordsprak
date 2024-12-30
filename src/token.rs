use regex::Regex;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug, EnumIter, Clone)]
pub enum Token {
    // Single character tokens
    Dot,

    // Literals
    String,

    Identifier,

    // Keywords
    Skriv,

    I,

    Eof,
}

impl Token {
    pub fn pattern(&self) -> Regex {
        match self {
            Token::Dot => Regex::new(r"\."),
            Token::String => Regex::new("\"[^\"]*\""),
            Token::Skriv => Regex::new(r"Skriv"),
            Token::Eof => Regex::new("\n"),
            Token::Identifier => Regex::new(".*"),
        }
        .expect("Failed to parse token regex")
    }

    pub fn matches(&self, string: &String, previous_tokens: &Vec<Token>) -> bool {
        match self {
            Token::Dot => Regex::new(r"\.")
                .expect("Failed to parse regex")
                .find(string)
                .is_some(),
            Token::String => Regex::new("\"[^\"]*\"")
                .expect("Failed to parse regex")
                .find(string)
                .is_some(),
            Token::Skriv => Regex::new(r"Skriv")
                .expect("Failed to parse regex")
                .find(string)
                .is_some(),
            Token::Eof => Regex::new("\n")
                .expect("Failed to parse regex")
                .find(string)
                .is_some(),
            Token::Identifier => {
                let is_identifier = match previous_tokens.last() {
                    Some(Token::I) => true,
                    Some(Token::Skriv) => true,
                    None => false,
                };

                Regex::new(".*")
                    .expect("Failed to parse regex")
                    .find(string)
                    .is_some()
            }
            Token::I => Regex::new(" i ")
                .expect("Failed to parse regex")
                .find(string)
                .is_some(),
        }
    }

    pub fn from_string(string: &String, previous_tokens: &Vec<Token>) -> Option<Token> {
        let all_tokens = Token::iter();

        for token in all_tokens {
            // if let Some(_) = token.pattern().find(string) {
            //     return Some(token.clone());
            // }

            if token.matches(string, previous_tokens) {
                return Some(token.clone());
            }
        }

        None
    }
}
