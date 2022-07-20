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

    /// Returns the nth character in the stream, if available, without advancing the stream.
    pub fn look_ahead(&self, n: usize) -> Option<char> {
        if self.index >= self.chars.len() {
            None
        } else {
            Some(self.chars[self.index + n])
        }
    }

    /// Returns the next character in the stream, if available.
    pub fn next(&mut self) -> Option<char> {
        let chr = self.peek();
        self.index += 1;
        chr
    }
}


const SEPARATORS: &str = "\t\n";
const CTRL_CHARACTERS: &str = "\\#[";
const END_CTRL: &str = " ()[]{}";


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

    /// Skips comments and whitespace.
    fn skip_whitespace(charstream: &mut Charstream) {
        while let Some(c) = charstream.peek() {
            if SEPARATORS.contains(c) {
                charstream.next();
            } else if c == '/' && charstream.look_ahead(1) == Some('/') {
                // This is a comment, advance the character stream until `\n`
                while let Some(chr) = charstream.peek() {
                    if chr == '\n' {
                        break;
                    } else {
                        charstream.next();
                    }
                }
            } else {
                // The next token is a valid token, return
                return;
            }
        }
        // EOF
    }

    /// Gets the next token from a character stream, if available.
    fn next_token(charstream: &mut Charstream) -> Option<Token> {
        Self::skip_whitespace(charstream);

        let nextchar = match charstream.next() {
            Some(n) => n,
            None => return None,
        };

        let mut current = String::from(nextchar);

        let token = match nextchar {
            '\\' => {
                // This is Cobalt's primary control operator.
                current = String::new();

                while let Some(c) = charstream.peek() {
                    if END_CTRL.contains(c) {
                        break;
                    }
                    charstream.next();
                    current.push(c);
                }

                Token::new(TokenType::Ctrl, current)
            },
            '#' => {
                // This represents a heading.
                current = String::new();
                let mut n = 1;
                while let Some(c) = charstream.peek() {
                    if c == '#' {
                        charstream.next();
                        n += 1;
                    } else if c == ' ' {
                        charstream.next();
                    } else {
                        break;
                    }
                }

                while let Some(c) = charstream.peek() {
                    if c == '\n' {
                        charstream.next();
                        break;
                    }
                    charstream.next();
                    current.push(c);
                }

                match n {
                    1 => Token::new(TokenType::H1, current),
                    2 => Token::new(TokenType::H2, current),
                    3 => Token::new(TokenType::H3, current),
                    4 => Token::new(TokenType::H4, current),
                    5 => Token::new(TokenType::H5, current),
                    6 => Token::new(TokenType::H6, current),
                    _ => todo!(),
                }
            },
            '[' => {
                // This is a hyperlink.
                current = String::new();
                while let Some(c) = charstream.peek() {
                    if c == ']' {
                        charstream.next();
                        break;
                    }
                    charstream.next();
                    current.push(c);
                }

                Token::new(TokenType::Bracket, current)
            },
            '(' => {
                // This is an ID or a URL.
                current = String::new();
                while let Some(c) = charstream.peek() {
                    if c == ')' {
                        charstream.next();
                        break;
                    }
                    charstream.next();
                    current.push(c);
                }

                Token::new(TokenType::Paren, current)
            },
            '{' => {
                // This is a class.
                current = String::new();
                while let Some(c) = charstream.peek() {
                    if c == '}' {
                        charstream.next();
                        break;
                    }
                    charstream.next();
                    current.push(c);
                }

                Token::new(TokenType::Brace, current)
            }
            _ => {
                while let Some(c) = charstream.peek() {
                    if CTRL_CHARACTERS.contains(c) {
                        break;
                    }
                    charstream.next();
                    current.push(c);
                }
                Token::new(TokenType::Paragraph, current)
            },
        };

        Some(token)
    }

    /// Peeks at the next token in the stream, if available.
    pub fn peek(&self) -> Option<Token> {
        if self.index >= self.tokens.len() {
            None
        } else {
            Some(self.tokens[self.index].to_owned())
        }
    }

    /// Peeks at the next token if available and throws an unexpected EOF file otherwise.
    pub fn peek_unwrap(&self) -> Token {
        match self.peek() {
            Some(t) => t,
            None => todo!(),
        }
    }

    /// Gets the next token in the stream, if available.
    pub fn next(&mut self) -> Option<Token> {
        let token = self.peek();
        self.index += 1;
        token
    }

    /// Gets the next token if available and throws an unexpected EOF file otherwise.
    pub fn next_unwrap(&mut self) -> Token {
        match self.next() {
            Some(t) => t,
            None => todo!(),
        }
    }

    /// Collects all tokens into a `Vec`.  This *does not* consume the tokenizer.
    pub fn collect(&self) -> Vec<Token> {
        self.tokens.to_owned()
    }
}