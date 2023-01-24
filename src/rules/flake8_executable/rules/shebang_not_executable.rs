use std::path::Path;

use ruff_macros::derive_message_formats;
use rustpython_ast::Location;

use crate::ast::types::Range;
use crate::define_violation;
use crate::fs::is_executable;
use crate::registry::Diagnostic;
use crate::rules::flake8_executable::helpers::ShebangDirective;
use crate::violation::Violation;

define_violation!(
    pub struct ShebangNotExecutable;
);
impl Violation for ShebangNotExecutable {
    #[derive_message_formats]
    fn message(&self) -> String {
        format!("Shebang is present but file is not executable")
    }
}

/// EXE001
pub fn shebang_not_executable(
    filepath: &Path,
    lineno: usize,
    shebang: &ShebangDirective,
) -> Option<Diagnostic> {
    if let ShebangDirective::Match(_, start, end, _) = shebang {
        if let Some(executable) = is_executable(filepath) {
            // Check if file is executable by anyone
            if executable {
                None
            } else {
                let diagnostic = Diagnostic::new(
                    ShebangNotExecutable,
                    Range::new(
                        Location::new(lineno + 1, *start),
                        Location::new(lineno + 1, *end),
                    ),
                );
                Some(diagnostic)
            }
        } else {
            None
        }
    } else {
        None
    }
}
