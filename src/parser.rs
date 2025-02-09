use crate::expression::Expression;
use crate::token::{Token, TokenType};

pub struct Parser {
    tokens: Vec<Token>,
}

impl Parser {
    pub fn new(tokens: &Vec<Token>) -> Self {
        Self {
            tokens: tokens.clone(),
        }
    }

    pub fn parse(&self) -> Expression {
        for token in &self.tokens {
            return match token.token_type {
                TokenType::Spara => Expression::Binary {
                    left: (token),
                    operator: (),
                    right: (),
                },
                _ => Expression::Literal,
            };
        }

        Expression::Literal
    }
}
