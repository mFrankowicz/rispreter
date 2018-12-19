use crate::lval::lval_def::*;

#[derive(PartialEq, Debug, Clone)]
pub struct LLambda {
    pub local_lenv: Box<Lenv>,
    pub formals: Box<Lval>,
    pub body: Box<Lval>
}

impl LLambda {
    pub fn new(formals: Lval, body: Lval) -> Self{
        LLambda {
            local_lenv: Box::new(Lenv::new()),
            formals: Box::new(formals),
            body: Box::new(body),
        }
    }
}
