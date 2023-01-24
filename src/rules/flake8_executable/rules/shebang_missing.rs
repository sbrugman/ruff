use std::path::Path;

use ruff_macros::derive_message_formats;

use crate::ast::types::Range;
use crate::define_violation;
use crate::fs::is_executable;
use crate::registry::Diagnostic;
use crate::violation::Violation;

define_violation!(
    pub struct ShebangMissingExecutableFile;
);
impl Violation for ShebangMissingExecutableFile {
    #[derive_message_formats]
    fn message(&self) -> String {
        format!("The file is executable but no shebang is present")
    }
}

/// EXE002
pub fn shebang_missing(filepath: &Path) -> Option<Diagnostic> {
    // Check if file is executable by anyone
    if let Some(executable) = is_executable(filepath) {
        if executable {
            let diagnostic = Diagnostic::new(ShebangMissingExecutableFile, Range::default());
            Some(diagnostic)
        } else {
            None
        }
    } else {
        None
    }
}
