use crate::lval::lval_builtin::Lbuiltin;
use crate::lval::lval_def::*;
use crate::lval::lval_error::Lerror;

use rispreter_parser::structure::{NumType, Prelude, Risp, TypedVec};

pub fn read(parsed: Option<Risp>) -> Lval {
    match parsed {
        Some(some) => match some {
            Risp::LComment => Lval::lval_sexpr(),
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
            Risp::LVec(v) => match v {
                TypedVec::NumVec(v) => Lval::lval_int_vec(v),
            },
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
                Prelude::Nil => Lval::lval_qexpr(),
                Prelude::Lambda => Lval::lval_fun(Lbuiltin::lbuiltin_lambda()),
                Prelude::Fun => Lval::lval_fun(Lbuiltin::lbuiltin_fun()),
                Prelude::Fst => Lval::lval_fun(Lbuiltin::lbuiltin_fst()),
                Prelude::Snd => Lval::lval_fun(Lbuiltin::lbuiltin_snd()),
                Prelude::Trd => Lval::lval_fun(Lbuiltin::lbuiltin_trd()),
                Prelude::Nth => Lval::lval_fun(Lbuiltin::lbuiltin_nth()),
                Prelude::Last => Lval::lval_fun(Lbuiltin::lbuiltin_last()),
                Prelude::Do => Lval::lval_fun(Lbuiltin::lbuiltin_do()),
                Prelude::Let => Lval::lval_fun(Lbuiltin::lbuiltin_let()),
                Prelude::Select => Lval::lval_fun(Lbuiltin::lbuiltin_select()),
                Prelude::Curry => Lval::lval_fun(Lbuiltin::lbuiltin_curry()),
                Prelude::Uncurry => Lval::lval_fun(Lbuiltin::lbuiltin_uncurry()),
                Prelude::Def => Lval::lval_fun(Lbuiltin::lbuiltin_def()),
                Prelude::Put => Lval::lval_fun(Lbuiltin::lbuiltin_put()),
                Prelude::List => Lval::lval_fun(Lbuiltin::lbuiltin_list()),
                Prelude::Head => Lval::lval_fun(Lbuiltin::lbuiltin_head()),
                Prelude::Tail => Lval::lval_fun(Lbuiltin::lbuiltin_tail()),
                Prelude::Eval => Lval::lval_fun(Lbuiltin::lbuiltin_eval()),
                Prelude::Join => Lval::lval_fun(Lbuiltin::lbuiltin_join()),
                Prelude::Cons => Lval::lval_fun(Lbuiltin::lbuiltin_cons()),
                Prelude::Take => Lval::lval_fun(Lbuiltin::lbuiltin_take()),
                Prelude::Drop => Lval::lval_fun(Lbuiltin::lbuiltin_drop()),
                Prelude::Split => Lval::lval_fun(Lbuiltin::lbuiltin_split()),
                Prelude::Elemen => Lval::lval_fun(Lbuiltin::lbuiltin_elemen()),
                Prelude::Map => Lval::lval_fun(Lbuiltin::lbuiltin_map()),
                Prelude::Filter => Lval::lval_fun(Lbuiltin::lbuiltin_filter()),
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
                Prelude::Not => Lval::lval_fun(Lbuiltin::lbuiltin_not()),
                Prelude::And => Lval::lval_fun(Lbuiltin::lbuiltin_and()),
                Prelude::Or => Lval::lval_fun(Lbuiltin::lbuiltin_or()),
                Prelude::Xor => Lval::lval_fun(Lbuiltin::lbuiltin_xor()),
            },
        },
        None => Lval::lval_err(Lerror::GenericError {
            msg: "Parser error".to_string(),
        }),
    }
}
