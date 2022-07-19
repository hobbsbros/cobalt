//! Provides a simple Cobalt tokenizer.

pub mod token;

use token::{Token, TokenType};

/// Provides `peek()` and `next()` methods on characters coming in from the Cobalt source file.
pub struct Charstream {
    chars: Vec<char>,
    index: usize,
}

impl Charstream {
    /// Constructs a new `Charstream` from a `String`.
    pub fn new(source: String) -> Self {
        Self {
            chars: source.chars().collect(),
            index: 0,
        }
    }

    /// Returns the next character in the stream, if available, without advancing the stream.
    pub fn peek(&self) -> Option<char> {
        if self.index >= self.chars.len() {
            None
        } else {
            Some(self.chars[self.index])
        }
    }

    /// Peeks at the next character in the stream, or, if it is not available, throw an unexpected EOF error.
    pub fn peek_unwrap(&mut self) -> char {
        match self.next() {
            Some(chr) => chr,
            None => todo!(),
        }
    }

    /// Returns the next character in the stream, if available.
    pub fn next(&mut self) -> Option<char> {
        let chr = self.peek();
        self.index += 1;
        chr
    }

    /// Returns the next character in the stream, or, if it is not available, throw an unexpected EOF error.
    pub fn next_unwrap(&mut self) -> char {
        match self.next() {
            Some(chr) => chr,
            None => todo!(),
        }
    }
}


/// Tokenizes the Cobalt source file and prepares it for parsing.
pub struct Tokenizer {
    tokens: Vec<Token>,
    index: usize,
}

impl Tokenizer {
    /// Constructs a new `Tokenizer` from a `String`.
    pub fn new(source: String) -> Self {
        let mut charstream = Charstream::new(source);
        let mut tokens = Vec::new();

        while let Some(t) = Self::next_token(&mut charstream) {
            tokens.push(t);
        }

        Self {
            tokens,
            index: 0,
        }
    }

    /// Gets the next token from a character stream, if available.
    fn next_token(charstream: &mut Charstream) -> Option<Token> {
        todo!()
    }
}