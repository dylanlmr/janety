#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub enum Type {
    Named(String),
    List(Box<Type>),
    Func(Box<Type>, Box<Type>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Num(f64),
    Str(String),
    Sym(String),
    List(Vec<Expr>),
    Call(Box<Expr>, Box<Expr>),
    Lambda(String, Box<Expr>),
    If {
        cond: Box<Expr>,
        then_branch: Box<Expr>,
        else_branch: Box<Expr>,
    },
}

#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub enum TopLevel {
    TypeSignature { name: String, ty: Type },
    Defn { name: String, body: Expr },
    Expr(Expr),
}
