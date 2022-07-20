//! Creates an interface for parselets.

use crate::parser::{
    Parser,
    Tokenizer,
    Token,
    Expression,
};

pub trait Parselet {
    fn parse(&self, parser: &Parser, tokenizer: &mut Tokenizer, token: Token) -> Expression;
}