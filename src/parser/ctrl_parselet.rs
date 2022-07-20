//! Provides a parselet for control sequences.

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

pub struct CtrlParselet;

impl Parselet for CtrlParselet {
    /// Parses a control sequence into an expression.
    fn parse(&self, _parser: &Parser, tokenizer: &mut Tokenizer, token: Token) -> Expression {
        // Get the keyword.
        let keyword = token.get_value();

        let mut class: Option<String> = None;
        let mut id: Option<String> = None;

        while let Some(t) = tokenizer.next() {
            match t.get_type() {
                TokenType::Paren => {
                    class = Some(t.get_value());
                },
                TokenType::Bracket => {
                    id = Some(t.get_value());
                },
                TokenType::Brace => {
                    let argument = t.get_value();
                    return Expression::Ctrl {
                        keyword,
                        class,
                        id,
                        argument,
                    };
                },
                _ => throw(Error::ExpectedOpen (t.get_value())),
            }
        }

        throw(Error::CouldNotParse (token.get_value()));
    }
}