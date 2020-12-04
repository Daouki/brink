#![feature(peekable_next_if)]

use std::time::Instant;

mod frontend;
mod source_file;

use frontend::{find_mixed_and_invalid_indentations, lexer::Lexer, parse_session::ParseSession};
use source_file::SourceFile;

fn main() {
    println!("brink compiler v{}", env!("CARGO_PKG_VERSION"));
    println!();

    let start_time = Instant::now();

    let args = std::env::args().collect::<Vec<String>>();

    let source_file_path = &args[1];
    let source_file = match SourceFile::read(source_file_path.clone()) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    };

    let tokens = Lexer::tokenize(&source_file);

    let mut parse_session = ParseSession::new(source_file);

    find_mixed_and_invalid_indentations(&mut parse_session, &tokens);
    if parse_session.has_errors() {
        terminate_compilation(start_time, &parse_session, 1);
    }

    #[cfg(debug_assertions)]
    {
        for (index, token) in tokens.into_iter().enumerate() {
            println!(
                "{:>4}\t({}-{})\t\t{:?}",
                index, token.span.start, token.span.end, token.kind
            )
        }
        println!();
    }

    terminate_compilation(start_time, &parse_session, 0);
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
