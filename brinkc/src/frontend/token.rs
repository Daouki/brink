/// Represents a single atomic unit of a language grammar.
#[derive(Copy, Clone, Debug)]
pub struct Token {
    pub kind: TokenKind,
    /// Start (inclusive) and end (exclusive) byte positions in source code.
    pub span: (usize, usize),
}

impl Token {
    pub fn with_length(kind: TokenKind, start: usize, length: usize) -> Self {
        Self {
            kind,
            span: (start, start + length),
        }
    }
}

/// Describes the kind of a lexeme.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum TokenKind {
    // The essence of an off-side rule language grammar.
    NewLine,
    Indent,
    Dedent,

    Identifier,
    Integer,

    Let,

    Equal,

    // Pseudo-tokens.
    Invalid,
    MixedIndentation,
    InvalidIndentation,
    EndOfFile,
}

pub fn get_keyword_kind(identifier: &str) -> Option<TokenKind> {
    match identifier {
        "let" => Some(TokenKind::Let),
        _ => None,
    }
}
