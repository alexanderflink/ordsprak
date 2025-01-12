use regex::Regex;

#[derive(Debug)]
pub enum TokenType {
    // Single character tokens
    Dot,

    // Literals
    String,

    Identifier,

    // Keywords
    Skriv,
    Spara,

    I,

    Eof,
}

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub content: String,
    pub length: usize,
}

impl Token {
    // This could probably be done by iterating over the enum variants instead
    pub fn from_string(source: &str) -> Option<Token> {
        match source {
            "Spara" | "spara" => Some(Token {
                token_type: TokenType::Spara,
                content: source.to_owned(),
                length: source.chars().count(),
            }),
            "Skriv" | "skriv" => Some(Token {
                token_type: TokenType::Skriv,
                content: source.to_owned(),
                length: source.chars().count(),
            }),
            "I" | "i" => Some(Token {
                token_type: TokenType::I,
                content: source.to_owned(),
                length: source.chars().count(),
            }),
            "." => Some(Token {
                token_type: TokenType::Dot,
                content: source.to_owned(),
                length: source.chars().count(),
            }),
            _ => {
                let string_regex =
                    Regex::new("^\"(.*)\"$").expect("Failed to compile String regex");
                let identifier_regex =
                    Regex::new("^\'(.*)\'$").expect("Failed to compile Identifier regex");

                if Regex::is_match(&string_regex, source) {
                    return Some(Token {
                        token_type: TokenType::String,
                        content: source.to_owned(),
                        length: source.chars().count(),
                    });
                }

                if Regex::is_match(&identifier_regex, source) {
                    return Some(Token {
                        token_type: TokenType::Identifier,
                        content: source.to_owned(),
                        length: source.chars().count(),
                    });
                }

                None
            }
        }
    }
}
