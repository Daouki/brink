#![feature(peekable_next_if)]

use std::time::Instant;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum IndentKind {
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

    #[cfg(debug_assertions)]
    {
        println!(
            "Indentation sequence: {:#?}",
            detect_indent_kind(&source_code)
        );
        println!();
    }

    println!(
        "compilation finished in {:.6}s",
        start_time.elapsed().as_secs_f32()
    );
}

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
