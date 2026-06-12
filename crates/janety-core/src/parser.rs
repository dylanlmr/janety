use chumsky::{
    IterParser, Parser,
    primitive::{any, choice, just, none_of},
    recursive::recursive,
    text,
};

use crate::ast::{Expr, TopLevel, Type};

pub fn ident_parser<'a>() -> impl Parser<'a, &'a str, String> + Clone {
    let sym_start = any().filter(|c: &char| c.is_ascii_alphabetic() || "-_?+*/!><$".contains(*c));
    let sym_continue =
        any().filter(|c: &char| c.is_ascii_alphanumeric() || "-_?+*/!><$".contains(*c));

    sym_start
        .then(sym_continue.repeated())
        .to_slice()
        .map(|s: &str| s.to_owned())
}

pub fn expr_parser<'a>() -> impl Parser<'a, &'a str, Expr> + Clone {
    recursive(|expr| {
        let num = text::int(10)
            .map(|s: &str| s.parse().unwrap())
            .map(Expr::Num);

        let str = none_of('"')
            .repeated()
            .to_slice()
            .delimited_by(just('"'), just('"'))
            .map(|s: &str| Expr::Str(s.to_owned()));

        let sym = ident_parser().map(Expr::Sym);

        let list = expr
            .clone()
            .repeated()
            .collect::<Vec<_>>()
            .delimited_by(just('['), just(']'))
            .map(Expr::List);

        let call = expr
            .clone()
            .then(expr.clone().repeated().at_least(1).collect::<Vec<_>>())
            .delimited_by(just('(').padded(), just(')').padded())
            .map(|(func, args)| {
                args.into_iter()
                    .fold(func, |acc, arg| Expr::Call(Box::new(acc), Box::new(arg)))
            });

        let if_stmt = text::keyword("if")
            .padded()
            .ignore_then(expr.clone())
            .then(expr.clone())
            .then(expr.clone())
            .delimited_by(just('(').padded(), just(')').padded())
            .map(|((cond, then_branch), else_branch)| Expr::If {
                cond: Box::new(cond),
                then_branch: Box::new(then_branch),
                else_branch: Box::new(else_branch),
            });

        choice((num, str, sym, list, if_stmt, call)).padded()
    })
}

pub fn type_parser<'a>() -> impl Parser<'a, &'a str, Type> + Clone {
    recursive(|typee| {
        let named = choice((
            text::keyword("number"),
            text::keyword("string"),
            text::keyword("boolean"),
        ))
        .padded()
        .map(|s: &str| Type::Named(s.to_owned()));

        let list = typee
            .clone()
            .delimited_by(just('[').padded(), just(']').padded())
            .map(|t| Type::List(Box::new(t)));

        let func = just("->")
            .padded()
            .ignore_then(typee.clone().repeated().at_least(2).collect::<Vec<_>>())
            .delimited_by(just('(').padded(), just(')').padded())
            .map(|mut types| {
                let ret = types.pop().unwrap();
                types
                    .into_iter()
                    .rev()
                    .fold(ret, |acc, ty| Type::Func(Box::new(ty), Box::new(acc)))
            });

        choice((named, list, func)).padded()
    })
}

pub fn top_level_parser<'a>() -> impl Parser<'a, &'a str, TopLevel> + Clone {
    let type_sig = just("::")
        .padded()
        .ignore_then(ident_parser().padded())
        .then(type_parser().padded())
        .delimited_by(just('(').padded(), just(')').padded())
        .map(|(name, ty)| TopLevel::TypeSignature { name, ty });

    let defn = just("defn")
        .padded()
        .ignore_then(ident_parser().padded())
        .then(
            ident_parser()
                .padded()
                .repeated()
                .collect::<Vec<_>>()
                .delimited_by(just('[').padded(), just(']').padded()),
        )
        .then(expr_parser().padded())
        .delimited_by(just('(').padded(), just(')').padded())
        .map(|((name, args), body)| {
            let desugared_body = args
                .into_iter()
                .rev()
                .fold(body, |acc, arg| Expr::Lambda(arg, Box::new(acc)));
            TopLevel::Defn {
                name,
                body: desugared_body,
            }
        });

    let expr = expr_parser().padded().map(TopLevel::Expr);

    choice((type_sig, defn, expr)).padded()
}

pub fn file_parser<'a>() -> impl Parser<'a, &'a str, Vec<TopLevel>> + Clone {
    top_level_parser().repeated().collect()
}
