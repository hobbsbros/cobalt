//! Provides a parselet for paragraphs.

use crate::parser::{
    Token,
    Tokenizer,
    Parser,
    Expression,
    Parselet,
};

pub struct ParagraphParselet;

impl Parselet for ParagraphParselet {
    /// Parses a paragraph into an expression.
    fn parse(&self, _parser: &Parser, _tokenizer: &mut Tokenizer, token: Token) -> Expression {
        Expression::Paragraph (token.get_value())
    }
}