//! Provides a parselet for headers.

use crate::{
    parser::{
        Token,
        TokenType,
        Tokenizer,
        Parser,
        Expression,
        Parselet,
    },
    error::{throw, Error},
};

pub struct HeaderParselet;

impl Parselet for HeaderParselet {
    /// Parses a header into an expression.
    fn parse(&self, _parser: &Parser, _tokenizer: &mut Tokenizer, token: Token) -> Expression {
        match token.get_type() {
            TokenType::H1 => Expression::H1 (token.get_value()),
            TokenType::H2 => Expression::H2 (token.get_value()),
            TokenType::H3 => Expression::H3 (token.get_value()),
            TokenType::H4 => Expression::H4 (token.get_value()),
            TokenType::H5 => Expression::H5 (token.get_value()),
            TokenType::H6 => Expression::H6 (token.get_value()),
            _ => throw(Error::ExpectedHeading (token.get_value()))
        }
    }
}