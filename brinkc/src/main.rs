#![feature(peekable_next_if)]

use std::time::Instant;

use frontend::{find_mixed_and_invalid_indentations, lexer::Lexer, parse_session::ParseSession};

mod frontend;

/// Holds information about the character sequence used to denote a block
/// or continuation of a statement.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum IndentKind {
    Tab,
    Spaces(u8),
}

fn main() {
    println!("brink compiler v{}", env!("CARGO_PKG_VERSION"));
    println!();

    let start_time = Instant::now();

    let args = std::env::args().collect::<Vec<String>>();

    let source_file_path = &args[1];
    let source_code = match std::fs::read_to_string(&source_file_path) {
        Ok(s) => s,
        Err(_) => {
            eprintln!(
                "error: failed to open the source file \"{}\"",
                &source_file_path
            );
            std::process::exit(1);
        }
    };

    let indent_kind = detect_indent_kind(&source_code);

    let tokens = Lexer::tokenize(&source_code, indent_kind);

    let mut parse_session = ParseSession::new(source_file_path.clone(), source_code, indent_kind);

    find_mixed_and_invalid_indentations(&mut parse_session, &tokens);
    if parse_session.has_errors() {
        terminate_compilation(start_time, &parse_session, 1);
    }

    #[cfg(debug_assertions)]
    {
        for (index, token) in tokens.into_iter().enumerate() {
            println!(
                "{:>4}\t({}-{})\t\t{:?}",
                index, token.span.0, token.span.1, token.kind
            )
        }
        println!();
    }

    terminate_compilation(start_time, &parse_session, 0);
}

/// Looks up the source code line-by-line for a first non-empty (containing
/// at least single non-whitespace character), indented (starting with either
/// a tab or sequence of spaces) line. The indentation mustn't be followed
/// by a whitespace. If the indentation starts with an n number of tabs, then
/// indentation kind is assumed to be using a single tab; if it starts with
/// n number of spaces then the indentation kind is assumed to be that number
/// of spaces.
///
/// Defaults to 2 spaces. Only true if no indentation was used in the entirety
/// of the given source code or no validly indented line of code was found.
fn detect_indent_kind(source_code: &String) -> IndentKind {
    for line in source_code.lines().into_iter() {
        if line.trim_start().is_empty() {
            continue;
        }

        let mut chars = line.chars().peekable();
        if let Some(_) = chars.next_if_eq(&'\t') {
            return IndentKind::Tab;
        }

        if let Some(_) = chars.next_if_eq(&' ') {
            let mut space_count = 1;
            while let Some(_) = chars.next_if_eq(&' ') {
                space_count += 1;
            }

            // If there are any whitespace characters after the space sequence,
            // then it's either a mixed indentation or some weird characters after
            // the space sequence. Regardless of that, it means that the spaces
            // do not indent any code and we should proceed to the next line.
            if let Some(_) = chars.next_if(|c| c.is_whitespace()) {
                continue;
            }

            return IndentKind::Spaces(space_count);
        }

        // If the first whitespace character in a line is neither a tab or
        // a space... Then I have no idea what's going on, let's just proceed
        // to the next line and pretend the shit never happened.
        continue;
    }

    // If the used indentation kind couldn't be guessed (either because there
    // are no indentations used in the code or no indentation used was valid),
    // just default to 2 spaces. That's pretty standard. And beautiful. Everybody
    // should use 2 space indentations in their ML-like code.
    IndentKind::Spaces(2)
}

fn terminate_compilation(start_time: Instant, session: &ParseSession, exit_code: i32) {
    println!(
        "compilation finished with {} errors and {} warnings in {:.6}s",
        session.error_count(),
        session.warning_count(),
        start_time.elapsed().as_secs_f32()
    );
    std::process::exit(exit_code);
}
