use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, violation};

use crate::checkers::ast::Checker;

/// https://stackoverflow.com/questions/18840880/will-using-list-comprehension-to-read-a-file-automagically-call-close
/// https://github.com/tmshlvck/linux/blob/f3e69428b5e26b0851d7ef4c15859cffebf2b9de/tools/perf/util/setup.py#L55
///
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
pub struct OpenInComprehension;

impl Violation for OpenInComprehension {
    #[derive_message_formats]
    fn message(&self) -> String {
        format!("xyz")
    }
}

/// PTH208
pub(crate) fn open_in_comprehension(checker: &mut Checker) {}
