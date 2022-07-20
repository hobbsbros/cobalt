//! Provides an abstraction over tokens and token types.

/// Enumerates the types of possible tokens.
#[derive(Clone, Copy, Debug)]
pub enum TokenType {
    Id,
    Class,
    Ctrl,
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
    Paragraph,
    Paren,
    Bracket,
    Brace,
}


/// Holds a token's type and value.
#[derive(Clone, Debug)]
pub struct Token {
    t: TokenType,
    v: String,
}

impl Token {
    /// Constructs a new `Token` from its type and value.
    pub fn new(t: TokenType, v: String) -> Self {
        Self {
            t,
            v,
        }
    }

    /// Gets the type of a token.
    pub fn get_type(&self) -> TokenType {
        self.t
    }

    /// Gets the value of a token.
    pub fn get_value(&self) -> String {
        self.v.to_owned()
    }
}