//! Provides an abstraction over tokens and token types.

/// Enumerates the types of possible tokens.
#[derive(Clone, Copy)]
pub enum TokenType {

}


/// Holds a token's type and value.
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