use crate::token::Token;

pub enum Expression<'a> {
    Unary {
        left: &'a Token,
        right: &'a Expression<'a>,
    },
    Binary {
        left: &'a Expression<'a>,
        operator: Token,
        right: &'a Expression<'a>,
    },
    Literal,
}
