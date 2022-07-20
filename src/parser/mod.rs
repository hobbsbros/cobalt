//! Provides a simple Cobalt parser.

pub mod parselet;
pub mod header_parselet;
pub mod ctrl_parselet;
pub mod paragraph_parselet;
pub mod hyperlink_parselet;

use std::collections::HashMap;

pub use crate::tokenizer::{
    Tokenizer,
    Token,
    TokenType,
};

pub use parselet::Parselet;
use header_parselet::HeaderParselet;
use ctrl_parselet::CtrlParselet;
use paragraph_parselet::ParagraphParselet;
use hyperlink_parselet::HyperlinkParselet;

/// Abstracts over different "expressions" in Cobalt.
#[derive(Clone, Debug)]
pub enum Expression {
    Ctrl {
        keyword: String,
        class: Option<String>,
        id: Option<String>,
        argument: String,  
    },
    Paragraph (String),
    Hyperlink {
        text: String,
        href: String,
    },
    H1 (String),
    H2 (String),
    H3 (String),
    H4 (String),
    H5 (String),
    H6 (String),
}


/// Provides a Cobalt parser.
pub struct Parser {
    parselets: HashMap<TokenType, Box<dyn Parselet>>,
}

impl Parser {
    /// Constructs a new parser from a tokenizer.
    pub fn new() -> Self {
        let mut parselets: HashMap<TokenType, Box<dyn Parselet>> = HashMap::new();

        // Declarative grammar begins here.
        parselets.insert(TokenType::H1, Box::new(HeaderParselet {}));
        parselets.insert(TokenType::H2, Box::new(HeaderParselet {}));
        parselets.insert(TokenType::H3, Box::new(HeaderParselet {}));
        parselets.insert(TokenType::H4, Box::new(HeaderParselet {}));
        parselets.insert(TokenType::H5, Box::new(HeaderParselet {}));
        parselets.insert(TokenType::H6, Box::new(HeaderParselet {}));
        parselets.insert(TokenType::Ctrl, Box::new(CtrlParselet {}));
        parselets.insert(TokenType::Paragraph, Box::new(ParagraphParselet {}));
        parselets.insert(TokenType::Bracket, Box::new(HyperlinkParselet {}));

        Self {
            parselets,
        }
    }

    /// Parses the next expression from the tokenizer.
    fn parse(&self, tokenizer: &mut Tokenizer) -> Option<Expression> {
        let token = tokenizer.next()?;

        let parselet = self.parselets.get(&token.get_type())?;

        Some(parselet.parse(self, tokenizer, token))
    }

    /// Consumes the tokenizer and returns a vector of expressions.
    pub fn parse_all(&self, tokenizer: &mut Tokenizer) -> Vec<Expression> {
        let mut expressions = Vec::new();

        while let Some(x) = self.parse(tokenizer) {
            expressions.push(x);
        }

        expressions
    }
}