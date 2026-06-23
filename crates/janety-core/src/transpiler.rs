use crate::ast::{Expr, TopLevel};

pub fn transpile(ast: &[TopLevel]) -> String {
    let prelude = r#"# --- Prelude : Currified Native Functions ---
(def +c (fn [x] (fn [y] (+ x y))))
(def -c (fn [x] (fn [y] (- x y))))
(def *c (fn [x] (fn [y] (* x y))))
(def /c (fn [x] (fn [y] (/ x y))))
(def tuple-c (fn [x] (fn [y] (tuple x ;y))))
(def slice-c (fn [x] (fn [y] (slice x y))))
# ----------------------------------------------"#;

    let body = ast.iter()
        .filter_map(|decl| match decl {
            TopLevel::TypeSignature { .. } => None,
            TopLevel::Defn { name, body } => {
                let sym_name = transpile_sym(name);
                
                let compiled_body = match body {
                    Expr::Lambda(arg, inner) => {
                        format!("(fn {} [{}] {})", sym_name, transpile_sym(arg), transpile_expr(inner))
                    }
                    _ => transpile_expr(body)
                };
                
                Some(format!("(def {} {})", sym_name, compiled_body))
            }
            TopLevel::Expr(expr) => Some(transpile_expr(expr)),
        })
        .collect::<Vec<_>>()
        .join("\n\n");

    format!("{}\n\n{}", prelude, body)
}

fn transpile_sym(s: &str) -> String {
    match s {
        "+" => "+c".to_string(),
        "-" => "-c".to_string(),
        "*" => "*c".to_string(),
        "/" => "/c".to_string(),
        "tuple" => "tuple-c".to_string(),
        "slice" => "slice-c".to_string(),
        _ => s.to_string(),
    }
}

fn transpile_expr(expr: &Expr) -> String {
    match expr {
        Expr::Num(n) => n.to_string(),
        Expr::Str(s) => format!("\"{}\"", s),
        Expr::Sym(s) => transpile_sym(s),
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
