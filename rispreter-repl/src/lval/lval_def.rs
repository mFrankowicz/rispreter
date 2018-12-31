use crate::lval::lval_builtin::*;
use crate::lval::lval_env::Lenv;
use crate::lval::lval_error::Lerror;
use crate::lval::lval_lambda::LLambda;
use std::collections::VecDeque;
use std::fmt;
use std::rc::Rc;
#[allow(non_camel_case_types)] // please
#[derive(PartialEq, Clone)]
pub enum LvalType {
    LVAL_ERR(Lerror),
    LVAL_NUM(f64),
    LVAL_SYM(String),
    LVAL_FUN(Lbuiltin),
    LVAL_LAMBDA(LLambda),
    LVAL_STRING(String),
    LVAL_BOOL(bool),
    LVAL_SEXPR,
    LVAL_QEXPR,
}

impl fmt::Display for LvalType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LvalType::LVAL_ERR(err) => write!(f, "error: \"{}\"", err),
            LvalType::LVAL_NUM(num) => write!(f, "{}", num),
            LvalType::LVAL_SYM(sym) => write!(f, "{}", sym),
            LvalType::LVAL_STRING(str) => write!(f, "\"{}\"", str),
            LvalType::LVAL_FUN(fun) => write!(f, "{:?}", fun),
            LvalType::LVAL_LAMBDA(lambda) => {
                writeln!(f, "body: {:?}", lambda.body.cell)?;
                writeln!(f, "formals: {:?}", lambda.formals.cell)
            }
            LvalType::LVAL_SEXPR => write!(f, "()"),
            LvalType::LVAL_QEXPR => write!(f, "{{}}"),
            LvalType::LVAL_BOOL(b) => write!(f, "{}", b),
        }
    }
}

impl fmt::Debug for LvalType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LvalType::LVAL_ERR(err) => write!(f, "error: \"{}\"", err),
            LvalType::LVAL_NUM(num) => write!(f, "{}", num),
            LvalType::LVAL_SYM(sym) => write!(f, "{}", sym),
            LvalType::LVAL_STRING(str) => write!(f, "\"{}\"", str),
            LvalType::LVAL_FUN(fun) => write!(f, "{:?}", fun),
            LvalType::LVAL_LAMBDA(lambda) => {
                writeln!(f, "body: {:?}", lambda.body.cell)?;
                writeln!(f, "formals: {:?}", lambda.formals.cell)
            }
            LvalType::LVAL_SEXPR => write!(f, "()"),
            LvalType::LVAL_QEXPR => write!(f, "{{}}"),
            LvalType::LVAL_BOOL(b) => write!(f, "{}", b),
        }
    }
}

#[derive(PartialEq, Clone)]
pub struct Lval {
    pub ltype: LvalType,
    pub cell: VecDeque<Box<Lval>>,
}

impl Lval {
    pub fn lval_num(num: f64) -> Lval {
        Lval {
            ltype: LvalType::LVAL_NUM(num),
            cell: VecDeque::new(),
        }
    }

    pub fn lval_err(err: Lerror) -> Lval {
        Lval {
            ltype: LvalType::LVAL_ERR(err),
            cell: VecDeque::new(),
        }
    }

    pub fn lval_sym(sym: String) -> Lval {
        Lval {
            ltype: LvalType::LVAL_SYM(sym),
            cell: VecDeque::new(),
        }
    }

    pub fn lval_string(str: String) -> Lval {
        Lval {
            ltype: LvalType::LVAL_STRING(str),
            cell: VecDeque::new(),
        }
    }

    pub fn lval_sexpr() -> Lval {
        Lval {
            ltype: LvalType::LVAL_SEXPR,
            cell: VecDeque::new(),
        }
    }

    pub fn lval_qexpr() -> Lval {
        Lval {
            ltype: LvalType::LVAL_QEXPR,
            cell: VecDeque::new(),
        }
    }

    pub fn lval_bool(b: bool) -> Lval {
        Lval {
            ltype: LvalType::LVAL_BOOL(b),
            cell: VecDeque::new(),
        }
    }

    pub fn lval_fun(func: Lbuiltin) -> Lval {
        Lval {
            ltype: LvalType::LVAL_FUN(func),
            cell: VecDeque::new(),
        }
    }

    pub fn lval_lambda(formals: Lval, body: Lval) -> Lval {
        Lval {
            ltype: LvalType::LVAL_LAMBDA(LLambda::new(formals, body)),
            cell: VecDeque::new(),
        }
    }

    pub fn lval_lambda_copy(env: Rc<Lenv>, formals: Lval, body: Lval) -> Lval {
        Lval {
            ltype: LvalType::LVAL_LAMBDA(LLambda::llambda_copy(env, formals, body)),
            cell: VecDeque::new(),
        }
    }

    pub fn add_cell(&mut self, lval: Lval) -> &mut Self {
        self.cell.push_back(Box::new(lval));
        self
    }

    pub fn lval_pop(&mut self) -> Lval {
        *self.cell.pop_front().unwrap()
    }

    pub fn lval_pop_with_index(&mut self, index: usize) -> Lval {
        *self.cell.remove(index).unwrap()
    }

    pub fn lval_take(&mut self, index: usize) -> Lval {
        let x = self.cell.remove(index).clone();
        self.cell.clear();
        *x.unwrap()
    }

    pub fn lval_split(&mut self, index: usize) -> Lval {
        self.cell = self.cell.split_off(index);
        self.clone()
    }

    pub fn lval_join(&mut self, other: &mut Lval) {
        self.cell.append(&mut other.cell);
    }
}

impl fmt::Display for Lval {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.ltype {
            LvalType::LVAL_SEXPR => {
                if self.cell.is_empty() {
                    write!(f, "{}", self.ltype)
                } else {
                    write!(f, "(")?;
                    for elem in self.cell.iter() {
                        write!(f, " {} ", elem)?;
                    }
                    write!(f, ")")
                }
            }
            LvalType::LVAL_QEXPR => {
                if self.cell.is_empty() {
                    write!(f, "{}", self.ltype)
                } else {
                    write!(f, "{{")?;
                    for elem in self.cell.iter() {
                        write!(f, " {} ", elem)?;
                    }
                    write!(f, "}}")
                }
            }
            _ => write!(f, "{}", self.ltype),
        }
    }
}

impl fmt::Debug for Lval {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.ltype {
            LvalType::LVAL_SEXPR => {
                if self.cell.is_empty() {
                    write!(f, "{}", self.ltype)
                } else {
                    write!(f, "(")?;
                    for elem in self.cell.iter() {
                        write!(f, " {} ", elem)?;
                    }
                    write!(f, ")")
                }
            }
            LvalType::LVAL_QEXPR => {
                if self.cell.is_empty() {
                    write!(f, "{}", self.ltype)
                } else {
                    write!(f, "{{")?;
                    for elem in self.cell.iter() {
                        write!(f, " {} ", elem)?;
                    }
                    write!(f, "}}")
                }
            }
            _ => write!(f, "{}", self.ltype),
        }
    }
}

impl From<Lval> for Option<String> {
    fn from(v: Lval) -> Option<String> {
        match v.ltype {
            LvalType::LVAL_STRING(str) => Some(str),
            LvalType::LVAL_SYM(sym) => Some(sym),
            LvalType::LVAL_ERR(err) => Some(err.to_string()),
            _ => None,
        }
    }
}

impl From<Lval> for Option<f64> {
    fn from(v: Lval) -> Option<f64> {
        match v.ltype {
            LvalType::LVAL_NUM(num) => Some(num),
            _ => None,
        }
    }
}

impl PartialEq<Lval> for f64 {
    fn eq(&self, other: &Lval) -> bool {
        match other.ltype {
            LvalType::LVAL_NUM(ref num) => self == num,
            _ => false,
        }
    }
}

impl PartialEq<Lval> for bool {
    fn eq(&self, other: &Lval) -> bool {
        match other.ltype {
            LvalType::LVAL_BOOL(ref b) => self == b,
            _ => false,
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn create_lval_type() {
        // let lval = Lval::lval_err("err".to_string());
        // assert_eq!(lval.ltype, LvalType::LVAL_ERR("err".to_string()));

        let lval = Lval::lval_num(1.0);
        assert_eq!(lval.ltype, LvalType::LVAL_NUM(1.0));

        let lval = Lval::lval_sym("sym".to_string());
        assert_eq!(lval.ltype, LvalType::LVAL_SYM("sym".to_string()));

        let lval = Lval::lval_string("str".to_string());
        assert_eq!(lval.ltype, LvalType::LVAL_STRING("str".to_string()));

        let lval = Lval::lval_sexpr();
        assert_eq!(lval.ltype, LvalType::LVAL_SEXPR);

        let lval = Lval::lval_qexpr();
        assert_eq!(lval.ltype, LvalType::LVAL_QEXPR);
    }

    // #[test]
    // fn test_lenv_def() {
    //     let k = String::from("x");
    //     let v = Box::new(Lval::lval_num(1.0));
    //     let mut lenv = Lenv::new();
    //     lenv.paren_env = Some(Box::new(Lenv::new()));
    //     lenv.def(k, v);
    //     assert_eq!(Some(&Box::new(Lval::lval_num(1.0))), lenv.paren_env.unwrap().get(&String::from("x")));
    // }

    // #[test]
    // fn test_lenv_def_and_local_lenv() {
    //     let k = String::from("x");
    //     let v = Box::new(Lval::lval_num(1.0));
    //     let v_local = Box::new(Lval::lval_string("local_sym".to_string()));
    //
    //     let mut lenv = Lenv::new();
    //
    //     lenv.put(String::from("x"), v_local);
    //
    //     lenv.paren_env = Some(Box::new(Lenv::new()));
    //     lenv.def(k, v);
    //     assert_eq!(Some(&Box::new(Lval::lval_num(1.0))), lenv.paren_env.clone().unwrap().get(&String::from("x")));
    //     assert_eq!(Some(&Box::new(Lval::lval_string("local_sym".to_string()))), lenv.get(&String::from("x")));
    // }

}
