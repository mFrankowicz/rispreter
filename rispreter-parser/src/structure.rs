#[derive(Debug, PartialEq, Copy, Clone)]
pub enum NumType {
    Int(i64),
    Float(f64),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Risp {
    LNumber(NumType),
    LString(String),
    LSymbol(String),
    LPrelude(Prelude),
    LSyntaxErr(String),
    LChar(char),
    LComment,
    LBool(bool),
    LVec(TypedVec),
    Sexpr(Vec<Risp>),
    Qexpr(Vec<Risp>),
}

#[derive(Debug, PartialEq, Clone)]
pub enum SymbolKind {
    User(String),
    Builtin(Prelude),
}

#[derive(Debug, PartialEq, Clone)]
pub enum TypedVec {
    NumVec(Vec<f64>),
}
// TODO: this:
// pub struct RispSyntaxErrorFormat<'a> {
//     header: &'a str,
//     body: &'a str,
//     hit: &'a str,
//     line: i32,
//     word: i32,
// }

// TODO:
#[derive(Debug, PartialEq, Clone)]
pub enum Prelude {
    Lambda,
    Def,
    Put,
    List,
    Head,
    Tail,
    Eval,
    Join,
    Cons,
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    If,
    Eq,
    Neq,
    Gt,
    Lt,
    Gte,
    Lte,
    Get,
}
