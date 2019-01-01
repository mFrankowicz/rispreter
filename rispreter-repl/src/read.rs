use crate::lval::lval_builtin::Lbuiltin;
use crate::lval::lval_def::*;
use crate::lval::lval_error::Lerror;

use rispreter_parser::structures::{NumType, Prelude, Risp, TypedVec};

pub fn read(parsed: Option<Risp>) -> Lval {
    match parsed {
        Some(some) => match some {
            Risp::Sexpr(lvals) => {
                let mut sexpr = Lval::lval_sexpr();
                for lval in lvals {
                    sexpr.add_cell(read(Some(lval)));
                }
                sexpr
            }
            Risp::Qexpr(lvals) => {
                let mut qexpr = Lval::lval_qexpr();
                for lval in lvals {
                    qexpr.add_cell(read(Some(lval)));
                }
                qexpr
            }
            Risp::LVec(v) => {
                match v {
                    TypedVec::NumVec(v) => Lval::lval_int_vec(v)
                }
            }
            Risp::LNumber(numtype) => match numtype {
                NumType::Float(f) => Lval::lval_num(f),
                NumType::Int(i) => Lval::lval_num(i as f64),
            },
            Risp::LSymbol(sym) => Lval::lval_sym(sym),
            Risp::LString(str) => Lval::lval_string(str),
            Risp::LChar(ch) => Lval::lval_char(ch),
            Risp::LBool(b) => Lval::lval_bool(b),
            Risp::LSyntaxErr(err) => Lval::lval_err(Lerror::GenericError { msg: err }),
            Risp::LPrelude(p) => match p {
                Prelude::Lambda => Lval::lval_fun(Lbuiltin::lbuiltin_lambda()),
                Prelude::Def => Lval::lval_fun(Lbuiltin::lbuiltin_def()),
                Prelude::Put => Lval::lval_fun(Lbuiltin::lbuiltin_put()),
                Prelude::List => Lval::lval_fun(Lbuiltin::lbuiltin_list()),
                Prelude::Head => Lval::lval_fun(Lbuiltin::lbuiltin_head()),
                Prelude::Tail => Lval::lval_fun(Lbuiltin::lbuiltin_tail()),
                Prelude::Eval => Lval::lval_fun(Lbuiltin::lbuiltin_eval()),
                Prelude::Join => Lval::lval_fun(Lbuiltin::lbuiltin_join()),
                Prelude::Cons => Lval::lval_fun(Lbuiltin::lbuiltin_cons()),
                Prelude::Add => Lval::lval_fun(Lbuiltin::lbuiltin_add()),
                Prelude::Sub => Lval::lval_fun(Lbuiltin::lbuiltin_sub()),
                Prelude::Mul => Lval::lval_fun(Lbuiltin::lbuiltin_mul()),
                Prelude::Div => Lval::lval_fun(Lbuiltin::lbuiltin_div()),
                Prelude::Mod => Lval::lval_fun(Lbuiltin::lbuiltin_mod()),
                Prelude::If => Lval::lval_fun(Lbuiltin::lbuiltin_if()),
                Prelude::Eq => Lval::lval_fun(Lbuiltin::lbuiltin_eq()),
                Prelude::Neq => Lval::lval_fun(Lbuiltin::lbuiltin_neq()),
                Prelude::Gt => Lval::lval_fun(Lbuiltin::lbuiltin_gt()),
                Prelude::Lt => Lval::lval_fun(Lbuiltin::lbuiltin_lt()),
                Prelude::Gte => Lval::lval_fun(Lbuiltin::lbuiltin_gte()),
                Prelude::Lte => Lval::lval_fun(Lbuiltin::lbuiltin_lte()),
                Prelude::Get => Lval::lval_fun(Lbuiltin::lbuiltin_get()),
            },
            _ => Lval::lval_err(Lerror::GenericError {
                msg: "incomplete".to_string(),
            }),
        },
        None => Lval::lval_err(Lerror::GenericError {
            msg: "Parser error".to_string(),
        }),
    }
}
