use std::f32::consts::E;

use ruff_diagnostics::{Diagnostic, Violation};
use ruff_macros::{derive_message_formats, violation};
use ruff_python_ast::{helpers::find_keyword, comparable::StmtExpr};
use rustpython_parser::ast::{
    Constant, Expr, ExprAttribute, ExprCall, ExprConstant, ExprName, Keyword, Ranged, Stmt,
    StmtAssign, WithItem,
};

use crate::checkers::ast::Checker;

/// Related to:
/// https://beta.ruff.rs/docs/rules/open-file-with-context-handler/
/// https://beta.ruff.rs/docs/rules/multiple-with-statements/

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
pub struct ReadBlock {
    kind: String,
}

impl Violation for ReadBlock {
    #[derive_message_formats]
    fn message(&self) -> String {
        let ReadBlock { kind } = self;
        let func = if kind == "string" {"read_text" } else {"read_bytes"};

        format!("Replace `with open(x) as f: y = f.read()` with `y = Path(x).{}()", func)
    }
}

fn mode_to_kind(mode: String) -> String {
    if mode.contains("b") {
        "bytes".to_string()
    } else {
        "string".to_string()
    }
}

fn mode_to_io(mode: String) -> String {
    if mode.contains("w") || mode.contains("a") {
        "write".to_string()
    } else {
        "read".to_string()
    }
}

fn args_to_mode(args: &Vec<Expr>, keywords: &Vec<Keyword>) -> String {
    // 1. Take first arg is available
    if args.len() > 0 {
        let arg = args.first().unwrap();
        if let Expr::Constant(ExprConstant {
            value: Constant::Str(c),
            ..
        }) = arg
        {
            c.as_str().to_string()
        } else {
            "r".to_string()
        }
    // 2. Keyword argument with name `mode`
    } else if keywords.len() > 0 {
        if let Some(kw) = find_keyword(keywords, "mode") {
            if let Expr::Constant(ExprConstant {
                value: Constant::Str(c),
                ..
            }) = &kw.value
            {
                c.as_str().to_string()
            } else {
                "r".to_string()
            }
        } else {
            "r".to_string()
        }
    // 3. Default: "r"
    } else {
        "r".to_string()
    }
}

/// PTH206
pub(crate) fn read_block(
    checker: &mut Checker,
    stmt: &Stmt,
    items: &Vec<WithItem>,
    body: &Vec<Stmt>,
) {
    // Reference implementation
    // https://github.com/dosisod/refurb/blob/master/refurb/checks/pathlib/write_text.py

    // Restrict to single item, single line body statements for now
    if items.len() != 1 || body.len() != 1 {
        return;
    }

    let with_item = items.first().unwrap();
    // as `fh`
    let Some(handler_id) = &with_item.optional_vars else {
        return;
    };
    let Expr::Name(ExprName {  id: optional_id, .. }) = handler_id.as_ref() else {
        return;
    };

    if let Expr::Call(ExprCall {
        args,
        keywords,
        func,
        ..
    }) = &with_item.context_expr
    {
        // TODO: also check Path.open()
        if !checker
            .semantic()
            .resolve_call_path(&func)
            .map_or(false, |call_path| {
                matches!(call_path.as_slice(), ["", "open"])
            })
        {
            return;
        }

        // Find mode
        let mode = args_to_mode(args, keywords);

        // Check body (depends on mode)
        if mode == "r" {
            // Match read
        } else {
            // Match write later
            return;
        }

        let body_stmt = body.first().unwrap();

        let body_expr = match body_stmt{
            Stmt::Assign(StmtAssign { targets, value, .. }) => {
                if mode != "r" {
                    None
                }
                // Assign to single value
                else if targets.len() != 1 {
                    None
                }
                else {
                    Some(value.as_ref())
                }
            },
            Stmt::Expr(StmtExpr { value, .. }) => {
                if mode != "w" {
                    None
                }
                else {
                    Some(value)
                }
            },
            _ => None,
        };

        if let Some(Expr::Call(ExprCall {
            func,
            args,
            keywords,
            ..
        })) = body_expr
        {
            // `.read()` should have empty args
            if args.len() != 0 || keywords.len() != 0 {
                return;
            }

            // check if the file handler is used
            if let Expr::Attribute(ExprAttribute { value, attr, .. }) = func.as_ref() {
                if let Expr::Name(ExprName { id, .. }) = value.as_ref() {
                    if id.as_str() != optional_id || !["read", "readlines"].contains(&attr.as_str()) {
                        return;
                    }
                } else {
                    return;
                }
            }
        } else {
            return;
        }

        checker.diagnostics.push(Diagnostic::new(
            ReadBlock {
                kind: "string".to_string(),
            },
            stmt.range(),
        ));
    } else {
        return;
    }
}
