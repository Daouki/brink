use crate::IndentKind;

pub struct ParseSession {
    source_file_path: String,
    source_code: String,
    indent_kind: IndentKind,
    error_count: u32,
    warning_count: u32,
}

impl ParseSession {
    pub fn new(source_file_path: String, source_code: String, indent_kind: IndentKind) -> Self {
        Self {
            source_file_path,
            source_code,
            indent_kind,
            error_count: 0,
            warning_count: 0,
        }
    }

    /// Gets the type of indentation used in the source code.
    pub fn indent_kind(&self) -> IndentKind {
        self.indent_kind
    }

    pub fn error<S: AsRef<str>>(&mut self, span: (usize, usize), message: S) {
        self.error_count += 1;

        eprintln!("error: {}", message.as_ref());
        eprintln!(" -> {}:{}-{}", self.source_file_path, span.0, span.1);
        println!();
    }

    pub fn warning<S: AsRef<str>>(&mut self, span: (usize, usize), message: S) {
        self.error_count += 1;

        eprintln!("warning: {}", message.as_ref());
        eprintln!(" -> {}:{}-{}", self.source_file_path, span.0, span.1);
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
