use crate::IndentKind;

pub mod lexer;
pub mod parse_session;
pub mod token;

use self::{
    parse_session::ParseSession,
    token::{Token, TokenKind},
};

pub fn find_mixed_and_invalid_indentations(session: &mut ParseSession, tokens: &Vec<Token>) {
    for token in tokens {
        match token.kind {
            TokenKind::MixedIndentation => {
                if let IndentKind::Spaces(spaces_per_indent) = session.indent_kind() {
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
                if let IndentKind::Spaces(spaces_per_indent) = session.indent_kind() {
                    session.error(
                        token.span,
                        format!(
                            "invalid number of spaces in indentation: expected {}, but found {}",
                            spaces_per_indent,
                            token.span.1 - token.span.0
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
