use super::env::TypeEnv;
use crate::ast::{Expr, TopLevel, Type};

pub fn check_types(ast: &[TopLevel]) -> Result<(), Vec<String>> {
    let mut env = TypeEnv::new();
    let mut errors: Vec<String> = Vec::new();

    env.insert(
        "+".into(),
        Type::Func(
            Box::new(Type::Named("number".into())),
            Box::new(Type::Func(
                Box::new(Type::Named("number".into())),
                Box::new(Type::Named("number".into())),
            )),
        ),
    );
    env.insert(
        "first".into(),
        Type::Func(
            Box::new(Type::List(Box::new(Type::Named("number".into())))),
            Box::new(Type::Named("number".into())),
        ),
    );
    env.insert(
        "slice".into(),
        Type::Func(
            Box::new(Type::List(Box::new(Type::Named("number".into())))),
            Box::new(Type::Func(
                Box::new(Type::Named("number".into())),
                Box::new(Type::List(Box::new(Type::Named("number".into())))),
            )),
        ),
    );
    env.insert(
        "empty?".into(),
        Type::Func(
            Box::new(Type::List(Box::new(Type::Named("number".into())))),
            Box::new(Type::Named("boolean".into())),
        ),
    );
    env.insert(
        "cons".into(),
        Type::Func(
            Box::new(Type::Named("number".into())),
            Box::new(Type::Func(
                Box::new(Type::List(Box::new(Type::Named("number".into())))),
                Box::new(Type::List(Box::new(Type::Named("number".into())))),
            )),
        ),
    );

    for decl in ast {
        if let TopLevel::TypeSignature { name, ty } = decl {
            env.insert(name.clone(), ty.clone());
        }
    }

    for decl in ast {
        match decl {
            TopLevel::Defn { name, body } => {
                let signature = env.get(name).cloned();

                if let Some(expected_ty) = signature {
                    if let Err(e) = check_expr(body, &mut env, &expected_ty) {
                        errors.push(format!("In function '{}': {}", name, e));
                    }
                } else {
                    errors.push(format!("Function '{}' lacks a type signature (::)", name));
                }
            }
            TopLevel::Expr(expr) => {
                if let Err(e) = infer_expr(expr, &mut env) {
                    errors.push(format!("Top-level error: {}", e));
                }
            }
            TopLevel::TypeSignature { .. } => {}
        }
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

fn check_expr(expr: &Expr, env: &mut TypeEnv, expected: &Type) -> Result<(), String> {
    match expr {
        Expr::Lambda(arg_name, body) => {
            if let Type::Func(arg_ty, ret_ty) = expected {
                env.push_scope();
                env.insert(arg_name.clone(), *arg_ty.clone());
                let res = check_expr(body, env, ret_ty);
                env.pop_scope();
                res
            } else {
                Err(format!(
                    "Expected {}, but got a lambda function",
                    format_type(expected)
                ))
            }
        }
        Expr::If {
            cond,
            then_branch,
            else_branch,
        } => {
            check_expr(cond, env, &Type::Named("boolean".into()))?;
            check_expr(then_branch, env, expected)?;
            check_expr(else_branch, env, expected)?;
            Ok(())
        }
        Expr::List(items) => {
            if let Type::List(expected_inner) = expected {
                for item in items {
                    check_expr(item, env, expected_inner)?;
                }
                Ok(())
            } else {
                Err(format!(
                    "Expected '{}', but got a list",
                    format_type(expected)
                ))
            }
        }
        _ => {
            let inferred = infer_expr(expr, env)?;
            if inferred != *expected {
                Err(format!(
                    "Type mismatch: expected '{}', but inferred '{}'",
                    format_type(expected),
                    format_type(&inferred)
                ))
            } else {
                Ok(())
            }
        }
    }
}

fn infer_expr(expr: &Expr, env: &mut TypeEnv) -> Result<Type, String> {
    match expr {
        Expr::Num(_) => Ok(Type::Named("number".into())),
        Expr::Str(_) => Ok(Type::Named("string".into())),
        Expr::Sym(name) => env
            .get(name)
            .cloned()
            .ok_or_else(|| format!("Unknown symbol: '{}'", name)),
        Expr::Lambda(_, _) => Err("Cannot infer type of an unannotated lambda.".into()),
        Expr::List(items) => {
            if items.is_empty() {
                Ok(Type::List(Box::new(Type::Named("any".into()))))
            } else {
                let first_ty = infer_expr(&items[0], env)?;
                for (i, item) in items.iter().enumerate().skip(1) {
                    let ty = infer_expr(item, env)?;
                    if ty != first_ty {
                        return Err(format!(
                            "Heterogeneous list at index {}: '{}' vs '{}'",
                            i,
                            format_type(&first_ty),
                            format_type(&ty)
                        ));
                    }
                }
                Ok(Type::List(Box::new(first_ty)))
            }
        }
        Expr::Call(func, arg) => {
            let func_ty = infer_expr(func, env)?;
            if let Type::Func(arg_ty, ret_ty) = func_ty {
                check_expr(arg, env, &arg_ty)?;
                Ok(*ret_ty)
            } else {
                Err(format!(
                    "Calling a non-function, found '{}'",
                    format_type(&func_ty)
                ))
            }
        }
        Expr::If {
            cond,
            then_branch,
            else_branch,
        } => {
            check_expr(cond, env, &Type::Named("boolean".into()))?;
            let t1 = infer_expr(then_branch, env)?;
            let t2 = infer_expr(else_branch, env)?;
            if t1 != t2 {
                Err(format!(
                    "If branches mismatch: '{}' vs '{}'",
                    format_type(&t1),
                    format_type(&t2)
                ))
            } else {
                Ok(t1)
            }
        }
    }
}

pub fn format_type(ty: &Type) -> String {
    match ty {
        Type::Named(s) => s.clone(),
        Type::List(inner) => format!("[{}]", format_type(inner)),
        Type::Func(arg, ret) => format!("(-> {} {})", format_type(arg), format_type(ret)),
    }
}
