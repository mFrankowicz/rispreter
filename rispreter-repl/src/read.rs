use crate::lval::lval_def::*;
use rispreter_parser::{Risp, NumType, SymbolKind};

pub fn read(parsed: Option<Risp>) -> Lval {
    match parsed {
        Some(some) => {
            match some {
                Risp::Sexpr(lvals) => {
                    let mut sexpr = Lval::lval_sexpr();
                    for lval in lvals {
                        sexpr.add_cell(read(Some(lval)));
                    };
                    sexpr
                },
                Risp::Qexpr(lvals) => {
                    let mut qexpr = Lval::lval_qexpr();
                    for lval in lvals {
                        qexpr.add_cell(read(Some(lval)));
                    };
                    qexpr
                },
                Risp::LNumber(numtype) => {
                    match numtype {
                        NumType::Float(f) => {
                            Lval::lval_num(f)
                        },
                        NumType::Int(i) => {
                            Lval::lval_num(i as f64)
                        }
                    }
                },
                Risp::LSymbol(skind) => {
                    match skind {
                        SymbolKind::User(sym) => {
                            Lval::lval_sym(sym)
                        },
                        _ => {
                            Lval::lval_err("prelude not implemented yet".to_string())
                        }
                    }
                },
                Risp::LString(str) => {
                    Lval::lval_string(str)
                },
                _ => {
                    Lval::lval_err("incomplete".to_string())
                }
            }
        },
        None => {
            Lval::lval_err("Parser error".to_string())
        }
    }

}
