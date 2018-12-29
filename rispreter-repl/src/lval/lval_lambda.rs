use crate::lval::lval_def::*;
use crate::lval::lval_env::Lenv;
use std::rc::Rc;
#[derive(Debug, Clone)]
pub struct LLambda {
    pub local_lenv: Rc<Lenv>,
    pub formals: Box<Lval>,
    pub body: Box<Lval>
}

impl LLambda {
    pub fn new(formals: Lval, body: Lval) -> Self{
        LLambda {
            local_lenv: Lenv::new(),
            formals: Box::new(formals),
            body: Box::new(body),
        }
    }

    pub fn llambda_copy(env: Rc<Lenv>, formals: Lval, body: Lval) -> Self {
        LLambda {
            local_lenv: env,
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
