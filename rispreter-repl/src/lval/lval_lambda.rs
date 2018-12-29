use crate::lval::lval_def::*;
use crate::lval::lval_env::Lenv;
use std::rc::Rc;
#[derive(Debug, Clone)]
pub struct LLambda {
    pub local_lenv: Box<Rc<Lenv>>,
    pub formals: Box<Lval>,
    pub body: Box<Lval>
}

impl LLambda {
    pub fn new(paren: &Rc<Lenv>, formals: Lval, body: Lval) -> Self{
        LLambda {
            local_lenv: Box::new(Lenv::from(paren)),
            formals: Box::new(formals),
            body: Box::new(body),
        }
    }

    pub fn llambda_copy(env: Rc<Lenv>, formals: Lval, body: Lval) -> Self {
        LLambda {
            local_lenv: Box::new(env),
            formals: Box::new(formals),
            body: Box::new(body),
        }
    }
}

impl PartialEq for LLambda {
    fn eq(&self, other: &LLambda) -> bool {
        if self.body == other.body &&
        self.formals == other.formals {
            true
        } else {
            false
        }
    }
}
