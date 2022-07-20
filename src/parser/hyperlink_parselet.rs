//! Provides a parselet for hyperlinks.

use crate::parser::{
    Token,
    TokenType,
    Tokenizer,
    Parser,
    Expression,
    Parselet,
};

pub struct HyperlinkParselet;

impl Parselet for HyperlinkParselet {
    /// Parses a hyperlink into an expression.
    fn parse(&self, _parser: &Parser, tokenizer: &mut Tokenizer, token: Token) -> Expression {
        // Get the text.
        let text = token.get_value();

        let next = tokenizer.next_unwrap();
        if next.get_type() == TokenType::Paren {
            let href = next.get_value();
            Expression::Hyperlink {
                text,
                href,
            }
        } else {
            todo!();
        }
    }
}