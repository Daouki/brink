mod analyze_file;

use analyze_file::*;

pub struct SourceFile {
    /// Path to the source file, as given to the compiler via the command line.
    pub file_path: String,
    /// The complete source code readen from the source file.
    pub source_code: String,
    /// Kind of indentation used in the source code.
    pub indent_kind: IndentKind,
}

impl SourceFile {
    pub fn read(file_path: String) -> Result<Self, String> {
        let source_code = match std::fs::read_to_string(&file_path) {
            Ok(s) => s,
            Err(_) => {
                eprintln!("error: failed to open the source file \"{}\"", &file_path);
                std::process::exit(1);
            }
        };

        let indent_kind = detect_indent_kind(&source_code);

        Ok(Self {
            file_path,
            source_code,
            indent_kind,
        })
    }

    pub fn read_span(&self, span: SourceSpan) -> &str {
        &self.source_code[span.start..span.end]
    }
}

/// Holds information about the character sequence used to denote a block
/// or continuation of a statement.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum IndentKind {
    Tab,
    Spaces(u8),
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct SourceSpan {
    pub start: usize,
    pub end: usize,
}

impl SourceSpan {
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }

    pub fn from_length(start: usize, length: usize) -> Self {
        Self {
            start,
            end: start + length,
        }
    }

    pub fn len(self) -> usize {
        self.end - self.start
    }
}
