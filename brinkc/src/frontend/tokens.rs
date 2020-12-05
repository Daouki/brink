use super::token::{Token, TokenKind};

pub struct Tokens {
    tokens: Vec<Token>,
    position: usize,
}

impl Tokens {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            position: 0,
        }
    }

    /// Gets the underlying collection of tokens.
    pub fn as_vec(&self) -> &Vec<Token> {
        &self.tokens
    }

    pub fn at_end(&self) -> bool {
        self.tokens[self.position].kind == TokenKind::EndOfFile
    }

    /// Gets the first unconsumed token without advancing.
    pub fn peek(&mut self) -> Token {
        self.tokens[self.position]
    }

    /// Gets the recently advanced token.
    pub fn previous(&mut self) -> Token {
        self.tokens[self.position - 1]
    }

    /// Gets the current token and advances to the next one.
    pub fn advance(&mut self) -> Token {
        if self.tokens[self.position].kind != TokenKind::EndOfFile {
            self.position += 1;
            self.tokens[self.position - 1]
        } else {
            self.tokens[self.position]
        }
    }

    pub fn check(&mut self, kind: TokenKind) -> bool {
        println!("checking {:?}", kind);
        self.tokens[self.position].kind == kind
    }

    pub fn consume(&mut self, kind: TokenKind) -> Option<Token> {
        if self.tokens[self.position].kind == kind {
            self.position += 1;
            Some(self.tokens[self.position - 1])
        } else {
            None
        }
    }
}
