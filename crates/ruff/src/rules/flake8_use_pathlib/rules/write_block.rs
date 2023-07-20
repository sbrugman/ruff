use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, violation};
use rustpython_parser::ast::Stmt;

use crate::checkers::ast::Checker;

/// ## What it does
///
/// ## Why is this bad?
///
/// ## Example
/// ```python
/// ```
///
/// Use instead:
/// ```python
/// ```
#[violation]
pub struct WriteBlock;

impl Violation for WriteBlock {
    #[derive_message_formats]
    fn message(&self) -> String {
        format!(
            "Replace `with` block for writing a file with `Path(file_name).write_text(content)`"
        )
    }
}

/// PTH207
pub(crate) fn write_block(checker: &mut Checker, stmt: &Stmt) {}
