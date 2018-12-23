#[macro_use]
extern crate nom;

use nom::{digit, alphanumeric, multispace, AsChar, anychar};

use std::str;
use std::str::FromStr;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum NumType {
    Int(i64),
    Float(f64),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Risp {
    LNumber(NumType),
    LString(String),
    LSymbol(SymbolKind),
    LSyntaxErr(String),
    LChar(char),
    LComment(String),
    Sexpr(Vec<Risp>),
    Qexpr(Vec<Risp>),
}

#[derive(Debug, PartialEq, Clone)]
pub enum SymbolKind {
    User(String),
    Builtin(Prelude),
}

// TODO: this:
// pub struct RispSyntaxErrorFormat<'a> {
//     header: &'a str,
//     body: &'a str,
//     hit: &'a str,
//     line: i32,
//     word: i32,
// }

#[derive(Debug, PartialEq, Clone)]
pub enum Prelude {
    Lambda,
    List,
    Head,
    Tail,
    Eval,
    Join,
    Var,
    Def,
    Put,
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    And,
    Or,
    Not,
    BitAnd,
    BitOr,
    BitNot,
    BitXor,
    Neg,
    Gtt,
    Ltt,
    Gte,
    Lte,
    Equ,
    Neq,
    If,
    Load,
    Print,
    Error,
}

named!(
    prelude_keywords<Prelude>,
    alt!(
    map!(tag!("\\"), {|_| Prelude::Lambda}) |
    map!(tag!("list"), {|_| Prelude::List}) |
    map!(tag!("head"), {|_| Prelude::Head}) |
    map!(tag!("tail"), {|_| Prelude::Tail}) |
    map!(tag!("eval"), {|_| Prelude::Eval}) |
    map!(tag!("join"), {|_| Prelude::Join}) |
    map!(tag!("var"), {|_| Prelude::Var}) |
    map!(tag!("def"), {|_| Prelude::Def}) |
    map!(tag!("put"), {|_| Prelude::Put}) |
    map!(tag!("+"), {|_| Prelude::Add}) |
    map!(tag!("-"), {|_| Prelude::Sub}) |
    map!(tag!("%"), {|_| Prelude::Mod}) |
    map!(tag!("*"), {|_| Prelude::Mul}) |
    map!(tag!("/"), {|_| Prelude::Div}) |
    map!(tag!("^"), {|_| Prelude::BitXor}) |
    map!(tag!("&"), {|_| Prelude::BitAnd}) |
    map!(tag!("|"), {|_| Prelude::BitOr}) |
    map!(tag!("!"), {|_| Prelude::BitNot}) |
    map!(tag!("&&"), {|_| Prelude::And}) |
    map!(tag!("||"), {|_| Prelude::Or}) |
    map!(tag!("!!"), {|_| Prelude::Not}) |
    map!(tag!("neg"), {|_| Prelude::Neg}) |
    map!(tag!("<"), {|_| Prelude::Ltt}) |
    map!(tag!(">"), {|_| Prelude::Gtt}) |
    map!(tag!(">="), {|_| Prelude::Gte}) |
    map!(tag!("<="), {|_| Prelude::Lte}) |
    map!(tag!("=="), {|_| Prelude::Equ}) |
    map!(tag!("!="), {|_| Prelude::Neq}) |
    map!(tag!("if"), {|_| Prelude::If}) |
    map!(tag!("load"), {|_| Prelude::Load}) |
    map!(tag!("print"), {|_| Prelude::Print}) |
    map!(tag!("error"), {|_| Prelude::Error})
    )
);

// symbol
// THOUGHTS:
// a symbol can be any sequence of chars listed below,
// as we gonna make some builtins symbols like '<', '+', '-' etc...
// we must think a better way of handle these special syms.
// the question is how to handle this in parse,
// if we gonna allow the user to shadow builtin syms like '+' or '<'
// we can allow any char to pass here, otherwise we make need separated
// list of special syms and the user wont have a option for shadowing

// TODO:
// 1: make this a more nom's way

named!(
    user_symbol<String>,
    map!(
        map_res!(
            is_a!("qwertyuiopasdfghjklçzxcvbnmQWERTYUIOPASDFGHJKLÇZXCVBNM1234567890_§?£¢¬~+-*/%&\\"), str::from_utf8),
            |s| {s.to_string()}
        )
);

// TODO: implement prelude enums
named!(
    symbol<SymbolKind>,
    alt!(
        map!(user_symbol, |u| {SymbolKind::User(u)}) |
        map!(user_symbol, |p| {SymbolKind::User(p)} ))
);

named!(
    lchar<Risp>,
    map!(
        anychar,
        |c: char| Risp::LChar(c.as_char())
    )
);

// string
// TODO:
// 1: add support to multiple line (eg: """ line 1
//                                          line 2 """)
named!(
    string<&str>,
    delimited!(
        char!('\"'),
        map_res!(
            escaped!(call!(alphanumeric), '\\', one_of!("\"n\\")),
            str::from_utf8
        ),
        char!('\"')
    )
);


named!(
    unsigned_float<f64>,
    map_res!(
        map_res!(
            recognize!(alt!(
                delimited!(digit, tag!("."), opt!(digit)) | delimited!(opt!(digit), tag!("."), digit)
            )),
            str::from_utf8
        ),
        FromStr::from_str
    )
);

named!(
    float<f64>,
    map!(
        pair!(opt!(alt!(tag!("+") | tag!("-"))), unsigned_float),
        |(sign, value): (Option<&[u8]>, f64)| sign
        .and_then(|s| if s[0] == b'-' { Some(-1f64)} else { None })
        .unwrap_or(1f64) * value
    )
);

// int64
named!(
    unsigned_int<i64>,
    map_res!(
        map_res!(
            recognize!(digit), str::from_utf8
        ),
        FromStr::from_str
    )
);

named!(
    int<i64>,
    map!(
        pair!(opt!(alt!(tag!("+") | tag!("-"))), unsigned_int),
        |(sign, value): (Option<&[u8]>, i64)| sign
        .and_then(|s| if s[0] == b'-' { Some(-1i64)} else { None })
        .unwrap_or(1i64) * value
    )
);

// number
named!(
    number<Risp>,
    alt!(
        float => {|f| Risp::LNumber(NumType::Float(f))} |
        int => {|i| Risp::LNumber(NumType::Int(i))}
    )
);


named!(
    sexpr<Vec<Risp>>,
    delimited!(
            terminated!(tag!("("), opt!(multispace)),
            separated_list!(multispace, lval),
            preceded!(opt!(multispace), tag!(")"))
    )
);

named!(
    qexpr<Vec<Risp>>,
    delimited!(
        terminated!(tag!("{"), opt!(multispace)),
        separated_list!(multispace, lval),
        preceded!(opt!(multispace), tag!("}"))
    )
);

// l-values
named!(
    lval<Risp>,
    alt!(
        number |
        string => {|sr| Risp::LString(String::from(sr))} |
        symbol => {|s| Risp::LSymbol(s)} |
        sexpr => {|sx| Risp::Sexpr(sx)} |
        qexpr => {|qx| Risp::Qexpr(qx)}
    )
);

pub fn parse_risp(input: &[u8]) -> Option<Risp> {
    let val = lval(input);
    // println!("Got parse: {:?}", val);
    match val {
        Ok(v) => Some(v.1),
        Err(e) => Some(Risp::LSyntaxErr(format!("{:?}", e))),
    }
}
