use crate::ast::{Expr, TopLevel};

pub fn transpile(ast: &[TopLevel]) -> String {
    ast.iter()
        .filter_map(|decl| match decl {
            TopLevel::TypeSignature { .. } => None,
            TopLevel::Defn { name, body } => {
                Some(format!("(def {} {})", name, transpile_expr(body)))
            }
            TopLevel::Expr(expr) => Some(transpile_expr(expr)),
        })
        .collect::<Vec<_>>()
        .join("\n\n")
}

fn transpile_expr(expr: &Expr) -> String {
    match expr {
        Expr::Num(n) => n.to_string(),
        Expr::Str(s) => format!("\"{}\"", s),
        Expr::Sym(s) => s.clone(),
        Expr::List(items) => {
            let items_str = items
                .iter()
                .map(transpile_expr)
                .collect::<Vec<_>>()
                .join(" ");
            format!("[{}]", items_str)
        }
        Expr::Call(func, arg) => {
            format!("({} {})", transpile_expr(func), transpile_expr(arg))
        }
        Expr::Lambda(arg, body) => {
            format!("(fn [{}] {})", arg, transpile_expr(body))
        }
        Expr::If {
            cond,
            then_branch,
            else_branch,
        } => {
            format!(
                "(if {} {} {})",
                transpile_expr(cond),
                transpile_expr(then_branch),
                transpile_expr(else_branch)
            )
        }
    }
}
