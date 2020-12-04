use std::{iter::Peekable, str::CharIndices};

use crate::source_file::{IndentKind, SourceFile};

use super::token::{self, Token, TokenKind};

macro_rules! tokenize_operator {
    ($lexer:ident, $start:ident, $c:ident, $short_lexeme:literal, $short_kind:ident) => {
        if $c == $short_lexeme {
            $lexer
                .tokens
                .push(Token::with_length(TokenKind::$short_kind, $start, 1));
            continue;
        }
    };
}
/// Holds the lexer state during parsing.
pub struct Lexer<'a> {
    source_code: Peekable<CharIndices<'a>>,
    tokens: Vec<Token>,
    indent_kind: IndentKind,
    current_indent: u8,
}

impl<'a> Lexer<'a> {
    /// Turns the given source code into a sequence of tokens.
    pub fn tokenize(source_file: &'a SourceFile) -> Vec<Token> {
        Self::tokenize_source_code(&source_file.source_code, source_file.indent_kind)
    }

    pub fn tokenize_source_code(source_code: &'a str, indent_kind: IndentKind) -> Vec<Token> {
        let mut lexer = Self {
            source_code: source_code.char_indices().peekable(),
            indent_kind,
            tokens: Vec::new(),
            current_indent: 0,
        };

        lexer.run(source_code.len());

        lexer.tokens
    }

    fn run(&mut self, source_code_length: usize) {
        while let Some((start, c)) = self.source_code.next() {
            if c == ' ' || c == '\t' || c == '\r' {
                continue;
            }

            if c == '\n' {
                self.tokenize_newline(start);
                continue;
            }

            if c.is_ascii_digit() {
                self.tokenize_integer(start);
                continue;
            }

            if c.is_alphabetic() {
                self.tokenize_identifier(start, c);
                continue;
            }

            tokenize_operator!(self, start, c, '=', Equal);

            self.add_token(TokenKind::Invalid, start, 1);
        }

        self.add_token(TokenKind::EndOfFile, source_code_length, 0);
    }

    fn tokenize_newline(&mut self, start: usize) {
        if let IndentKind::Spaces(spaces_per_indent) = self.indent_kind {
            self.tokenize_newline_using_spaces(start, spaces_per_indent);
        } else {
            self.tokenize_newline_using_tabs(start);
        }
    }

    fn tokenize_newline_using_spaces(&mut self, start: usize, spaces_per_indent: u8) {
        // We expect either LF or CR LF line ending. LF CR line endings are not
        // supported. We should probably detect them in file analysis pass. Same
        // thing applies to indentation using tabs.

        let mut space_count = 0;
        while let Some(_) = self.source_code.next_if(|(_, c)| *c == ' ') {
            space_count += 1;
        }

        if let Some(_) = self.source_code.next_if(|(_, c)| *c == '\t') {
            self.add_token(TokenKind::NewLine, start, 1);
            self.add_token(TokenKind::MixedIndentation, start + 1, 1);
            return;
        }

        let indent_count = space_count / spaces_per_indent;
        if indent_count > self.current_indent {
            // TODO: Correct position for indents.
            for _ in 0..indent_count - self.current_indent {
                self.add_token(TokenKind::Indent, start, 1);
            }
        } else if indent_count < self.current_indent {
            // TODO: Correct position for dedents - they should cover all
            // of the next line starting whitespace, I believe.
            for _ in 0..self.current_indent - indent_count {
                self.add_token(TokenKind::Dedent, start, 1);
            }
        } else {
            self.add_token(TokenKind::NewLine, start, 1);
        }

        // TODO: Correct source code span for invalid indentation.
        let trailing_space_count = (space_count % spaces_per_indent) as usize;
        if space_count % spaces_per_indent != 0 {
            self.add_token(TokenKind::InvalidIndentation, start, trailing_space_count)
        }

        self.current_indent = indent_count;
    }

    fn tokenize_newline_using_tabs(&mut self, start: usize) {
        // See the first comment in the Self::tokenize_newline_using_spaces function.

        let mut tab_count: u8 = 0;
        while let Some(_) = self.source_code.next_if(|(_, c)| *c == '\t') {
            tab_count += 1;
        }

        let mut space_count = 0;
        while let Some(_) = self.source_code.next_if(|(_, c)| *c == ' ') {
            space_count += 1;
        }
        if space_count > 0 {
            self.add_token(TokenKind::NewLine, start, 1);
            self.add_token(
                TokenKind::MixedIndentation,
                start + tab_count as usize + 1,
                space_count,
            );
            return;
        }

        if tab_count > self.current_indent {
            // TODO: Correct position for indents.
            for _ in 0..tab_count - self.current_indent {
                self.add_token(TokenKind::Indent, start, 1);
            }
        } else if tab_count < self.current_indent {
            for _ in 0..self.current_indent - tab_count {
                self.add_token(TokenKind::Dedent, start + 1, tab_count as usize);
            }
        } else {
            self.add_token(TokenKind::NewLine, start, 1);
        }
        self.current_indent = tab_count
    }

    fn tokenize_integer(&mut self, start: usize) {
        let mut length = 1;
        while let Some(_) = self.source_code.next_if(|(_, c)| c.is_ascii_digit()) {
            length += 1;
        }

        self.add_token(TokenKind::Integer, start, length);
    }

    fn tokenize_identifier(&mut self, start: usize, first_char: char) {
        let mut identifier = String::from(first_char);
        while let Some((_, c)) = self.source_code.next_if(|(_, c)| c.is_alphanumeric()) {
            identifier.push(c);
        }

        if let Some(keyword) = token::get_keyword_kind(&identifier) {
            self.add_token(keyword, start, identifier.len());
        } else {
            self.add_token(TokenKind::Identifier, start, identifier.len());
        }
    }

    fn add_token(&mut self, kind: TokenKind, start: usize, length: usize) {
        self.tokens.push(Token::with_length(kind, start, length));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ignores_whitespace() {
        // The lexer ignores all starting whitespaces on the first line of source code.
        let input = "   \t\t\r\r  \t";
        let result = Lexer::tokenize_source_code(input, IndentKind::Tab);

        assert_eq!(1, result.len());
        assert_eq!(TokenKind::EndOfFile, result[0].kind);
    }

    #[test]
    fn tokenizes_newline_using_spaces() {
        let input = "\n  \n  \n";
        let result = Lexer::tokenize_source_code(input, IndentKind::Spaces(2))[1];

        assert_eq!(TokenKind::NewLine, result.kind);
    }

    #[test]
    fn tokenizes_indent_using_spaces() {
        let input = "\n  \n";
        let result = Lexer::tokenize_source_code(input, IndentKind::Spaces(2))[0];

        assert_eq!(TokenKind::Indent, result.kind);
    }

    #[test]
    fn tokenizes_double_indent_using_spaces() {
        let input = "\n    \n";
        let result = Lexer::tokenize_source_code(input, IndentKind::Spaces(2));

        assert_eq!(TokenKind::Indent, result[0].kind);
        assert_eq!(TokenKind::Indent, result[1].kind);
    }

    #[test]
    fn tokenizes_dedent_using_spaces() {
        let input = "\n  \n\n";
        let result = Lexer::tokenize_source_code(input, IndentKind::Spaces(2))[1];

        assert_eq!(TokenKind::Dedent, result.kind);
    }

    #[test]
    fn tokenizes_double_dedent_using_spaces() {
        let input = "\n    \n\n";
        let result = Lexer::tokenize_source_code(input, IndentKind::Spaces(2));

        assert_eq!(TokenKind::Dedent, result[2].kind);
        assert_eq!(TokenKind::Dedent, result[3].kind);
    }

    #[test]
    fn tokenizes_newline_using_tabs() {
        let input = "\n\t\n\t\n";
        let result = Lexer::tokenize_source_code(input, IndentKind::Tab)[1];

        assert_eq!(TokenKind::NewLine, result.kind);
    }

    #[test]
    fn tokenizes_indent_using_tabs() {
        let input = "\n\t\n";
        let result = Lexer::tokenize_source_code(input, IndentKind::Tab)[0];

        assert_eq!(TokenKind::Indent, result.kind);
    }

    #[test]
    fn tokenizes_double_indent_using_tabs() {
        let input = "\n\t\t\n";
        let result = Lexer::tokenize_source_code(input, IndentKind::Tab);

        assert_eq!(TokenKind::Indent, result[0].kind);
        assert_eq!(TokenKind::Indent, result[1].kind);
    }

    #[test]
    fn tokenizes_dedent_using_tabs() {
        let input = "\n\t\n\n";
        let result = Lexer::tokenize_source_code(input, IndentKind::Tab)[1];

        assert_eq!(TokenKind::Dedent, result.kind);
    }

    #[test]
    fn tokenizes_double_dedent_using_tabs() {
        let input = "\n\t\t\n\n";
        let result = Lexer::tokenize_source_code(input, IndentKind::Tab);

        assert_eq!(TokenKind::Dedent, result[2].kind);
        assert_eq!(TokenKind::Dedent, result[3].kind);
    }

    #[test]
    fn tokenizes_mixed_indentation_tabs_when_expected_spaces() {
        let input = "\n\t\n";
        let result = Lexer::tokenize_source_code(input, IndentKind::Spaces(2))[1];

        assert_eq!(TokenKind::MixedIndentation, result.kind);
    }

    #[test]
    fn tokenizes_mixed_indentation_spaces_when_expected_tabs() {
        let input = "\n  \n";
        let result = Lexer::tokenize_source_code(input, IndentKind::Tab)[1];

        assert_eq!(TokenKind::MixedIndentation, result.kind);
    }

    #[test]
    fn tokenizes_invalid_indentation() {
        let input = "\n   \n";
        let result = Lexer::tokenize_source_code(input, IndentKind::Spaces(2))[1];

        assert_eq!(TokenKind::InvalidIndentation, result.kind);
    }

    #[test]
    fn tokenizes_integer() {
        let input = "1234";
        let result = Lexer::tokenize_source_code(input, IndentKind::Tab)[0];

        assert_eq!(TokenKind::Integer, result.kind);
        assert_eq!(input.len(), result.span.len());
    }

    #[test]
    fn tokenizes_short_operator() {
        let input = "=";
        let result = Lexer::tokenize_source_code(input, IndentKind::Tab)[0];

        assert_eq!(TokenKind::Equal, result.kind)
    }
    #[test]
    fn tokenizes_invalid() {
        let input = "@";
        let result = Lexer::tokenize_source_code(input, IndentKind::Tab)[0];

        assert_eq!(TokenKind::Invalid, result.kind);
    }
}
