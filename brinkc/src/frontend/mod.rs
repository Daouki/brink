use crate::source_file::IndentKind;

pub mod lexer;
pub mod parse_session;
pub mod parser;
pub mod token;
pub mod tokens;

use self::{parse_session::ParseSession, token::TokenKind, tokens::Tokens};

pub fn find_mixed_and_invalid_indentations(session: &mut ParseSession, tokens: &Tokens) {
    for token in tokens.as_vec() {
        match token.kind {
            TokenKind::MixedIndentation => {
                if let IndentKind::Spaces(spaces_per_indent) = session.source_file.indent_kind {
                    session.error(
                        token.span,
                        format!(
                            "mixed indentation: expected {} spaces, but found '\\t'",
                            spaces_per_indent
                        ),
                    );
                } else {
                    session.error(
                        token.span,
                        format!("mixed indentation; expected tab, but found ' '"),
                    )
                }
            }

            TokenKind::InvalidIndentation => {
                if let IndentKind::Spaces(spaces_per_indent) = session.source_file.indent_kind {
                    session.error(
                        token.span,
                        format!(
                            "invalid number of spaces in indentation: expected {}, but found {}",
                            spaces_per_indent,
                            token.span.len(),
                        ),
                    );
                } else {
                    panic!();
                }
            }

            _ => {}
        }
    }
}
