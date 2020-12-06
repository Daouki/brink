use crate::source_file::{SourceFile, SourceSpan};

pub struct ParseSession {
    pub source_file: SourceFile,
    error_count: u32,
    warning_count: u32,
}

impl ParseSession {
    pub fn new(source_file: SourceFile) -> Self {
        Self {
            source_file,
            error_count: 0,
            warning_count: 0,
        }
    }

    pub fn error<S: AsRef<str>>(&mut self, span: SourceSpan, message: S) {
        self.error_count += 1;

        eprintln!("error: {}", message.as_ref());
        eprintln!(
            " -> {}:{}-{}",
            self.source_file.file_path, span.start, span.end
        );
        println!();
    }

    #[allow(dead_code)]
    pub fn warning<S: AsRef<str>>(&mut self, span: SourceSpan, message: S) {
        self.error_count += 1;

        eprintln!("warning: {}", message.as_ref());
        eprintln!(
            " -> {}:{}-{}",
            self.source_file.file_path, span.start, span.end
        );
        println!();
    }

    pub fn error_count(&self) -> u32 {
        self.error_count
    }

    pub fn warning_count(&self) -> u32 {
        self.warning_count
    }

    pub fn has_errors(&self) -> bool {
        self.error_count != 0
    }
}
