use crate::lval::lval_eval::*;
use crate::lval::lval_def::*;
use crate::read::read;
use rispreter_parser::parse_risp;

pub fn eval_rispreter(lenv: &mut Lenv, input: String) -> Lval {
    lval_eval(lenv, &mut read(parse_risp(input.as_bytes())))
}
