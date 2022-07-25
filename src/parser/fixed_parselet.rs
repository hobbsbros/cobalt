//! Provides a parselet for fixed `div`s.

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

pub struct FixedParselet;

impl Parselet for FixedParselet {
    /// Parses a fixed `div` into an expression.
    fn parse(&self, parser: &Parser, tokenizer: &mut Tokenizer, _token: Token) -> Expression {
        let mut expressions = Vec::new();

        while let Some(t) = tokenizer.peek() {
            if t.get_type() == TokenType::Fixed {
                tokenizer.next();
                break;
            }

            // Parse the next expression and pause
            let expr = match parser.parse(tokenizer) {
                Some(e) => e,
                None => throw(Error::CouldNotParse (t.get_value())),
            };
            expressions.push(expr);
        }

        Expression::Fixed (expressions)
    }
}