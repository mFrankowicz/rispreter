use crate::lval::lval_def::*;
use crate::lval::lval_env::Lenv;
use crate::lval::lval_eval;
use crate::lval::lval_error::{LvalTypeMeta, Lerror};
use std::rc::Rc;
//use crate::lval::lval_lambda::LLambda;

pub struct Lbuiltin(pub fn(lenv: &Rc<Lenv>, lval: &mut Lval) -> Lval, String);

impl Lbuiltin {
    pub fn add_builtins(lenv: &Rc<Lenv>) {
        lenv.add_builtin("\\", Lbuiltin::lbuiltin_lambda());
        lenv.add_builtin("def", Lbuiltin::lbuiltin_def());
        lenv.add_builtin("=", Lbuiltin::lbuiltin_put());

        lenv.add_builtin("list", Lbuiltin::lbuiltin_list());
        lenv.add_builtin("head", Lbuiltin::lbuiltin_head());
        lenv.add_builtin("tail", Lbuiltin::lbuiltin_tail());
        lenv.add_builtin("eval", Lbuiltin::lbuiltin_eval());
        lenv.add_builtin("join", Lbuiltin::lbuiltin_join());
        lenv.add_builtin("cons", Lbuiltin::lbuiltin_cons());

        lenv.add_builtin("+", Lbuiltin::lbuiltin_add());
        lenv.add_builtin("-", Lbuiltin::lbuiltin_sub());
        lenv.add_builtin("*", Lbuiltin::lbuiltin_mul());
        lenv.add_builtin("/", Lbuiltin::lbuiltin_div());
        lenv.add_builtin("%", Lbuiltin::lbuiltin_mod());

        lenv.add_builtin("if", Lbuiltin::lbuiltin_if());
        lenv.add_builtin("eq", Lbuiltin::lbuiltin_eq());
        lenv.add_builtin("neq", Lbuiltin::lbuiltin_neq());
        lenv.add_builtin("gt", Lbuiltin::lbuiltin_gt());
        lenv.add_builtin("lt", Lbuiltin::lbuiltin_lt());
        lenv.add_builtin("gte", Lbuiltin::lbuiltin_gte());
        lenv.add_builtin("lte", Lbuiltin::lbuiltin_lte());
    }

    fn lbuiltin_add() -> Lbuiltin {
        Lbuiltin(add, "+".to_string())
    }

    fn lbuiltin_sub() -> Lbuiltin {
        Lbuiltin(sub, "-".to_string())
    }

    fn lbuiltin_mul() -> Lbuiltin {
        Lbuiltin(mul, "*".to_string())
    }

    fn lbuiltin_div() -> Lbuiltin {
        Lbuiltin(div, "/".to_string())
    }

    fn lbuiltin_mod() -> Lbuiltin {
        Lbuiltin(modl, "mod".to_string())
    }

    fn lbuiltin_head() -> Lbuiltin {
        Lbuiltin(head, "head".to_string())
    }

    fn lbuiltin_tail() -> Lbuiltin {
        Lbuiltin(tail, "tail".to_string())
    }

    fn lbuiltin_list() -> Lbuiltin {
        Lbuiltin(list, "list".to_string())
    }

    fn lbuiltin_join() -> Lbuiltin {
        Lbuiltin(join, "join".to_string())
    }

    fn lbuiltin_cons() -> Lbuiltin {
        Lbuiltin(cons, "cons".to_string())
    }

    fn lbuiltin_eval() -> Lbuiltin {
        Lbuiltin(eval, "eval".to_string())
    }

    fn lbuiltin_def() -> Lbuiltin {
        Lbuiltin(def, "def".to_string())
    }

    fn lbuiltin_put() -> Lbuiltin {
        Lbuiltin(put, "=".to_string())
    }

    fn lbuiltin_lambda() -> Lbuiltin {
        Lbuiltin(lambda, "\\".to_string())
    }

    fn lbuiltin_eq() -> Lbuiltin {
        Lbuiltin(eq, "eq".to_string())
    }

    fn lbuiltin_neq() -> Lbuiltin {
        Lbuiltin(neq, "neq".to_string())
    }

    fn lbuiltin_gt() -> Lbuiltin {
        Lbuiltin(gt, "gt".to_string())
    }

    fn lbuiltin_lt() -> Lbuiltin {
        Lbuiltin(lt, "lt".to_string())
    }

    fn lbuiltin_gte() -> Lbuiltin {
        Lbuiltin(gte, "gte".to_string())
    }

    fn lbuiltin_lte() -> Lbuiltin {
        Lbuiltin(lte, "lte".to_string())
    }

    fn lbuiltin_if() -> Lbuiltin {
        Lbuiltin(lif, "if".to_string())
    }
}

impl PartialEq for Lbuiltin {
    fn eq(&self, other: &Self) -> bool {
        self.0 as usize == other.0 as usize
    }
}

impl Clone for Lbuiltin {
    fn clone(&self) -> Self {
        Lbuiltin(self.0.clone(), self.1.clone())
    }
}

impl std::fmt::Debug for Lbuiltin {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "prelude({})", self.1)
    }
}

// builtins

/// Add n arguments
/// # Examples
/// ```
/// # use rispreter_repl::lval::lval_env::Lenv;
/// # use rispreter_repl::eval::eval_rispreter;
/// # use rispreter_repl::lval::lval_builtin::*;
/// # use rispreter_repl::lval::lval_def::*;
/// let mut builtins = Lenv::new();
/// Lbuiltin::add_builtins(&mut builtins);
///
/// let res = eval_rispreter(&mut builtins, "(+ 1 2)".to_string());
/// assert_eq!(3f64, res);
/// ```
fn add(lenv: &Rc<Lenv>, lval: &mut Lval) -> Lval {
    op(lenv, lval, '+')
}

/// Subtracts n arguments
/// # Examples
/// ```
/// # use rispreter_repl::lval::lval_env::Lenv;
/// # use rispreter_repl::eval::eval_rispreter;
/// # use rispreter_repl::lval::lval_builtin::*;
/// # use rispreter_repl::lval::lval_def::*;
/// let mut builtins = Lenv::new();
/// Lbuiltin::add_builtins(&mut builtins);
///
/// let res = eval_rispreter(&mut builtins, "(- 3 2)".to_string());
/// assert_eq!(1f64, res);
/// ```
fn sub(lenv: &Rc<Lenv>, lval: &mut Lval) -> Lval {
    op(lenv, lval, '-')
}

/// Multiply n arguments
/// # Examples
/// ```
/// # use rispreter_repl::lval::lval_env::Lenv;
/// # use rispreter_repl::eval::eval_rispreter;
/// # use rispreter_repl::lval::lval_builtin::*;
/// # use rispreter_repl::lval::lval_def::*;
/// let mut builtins = Lenv::new();
/// Lbuiltin::add_builtins(&mut builtins);
///
/// let res = eval_rispreter(&mut builtins, "(* 2 3)".to_string());
/// assert_eq!(6f64, res);
/// ```
fn mul(lenv: &Rc<Lenv>, lval: &mut Lval) -> Lval {
    op(lenv, lval, '*')
}

/// Divides n arguments
/// returns a `LvalType::LVAL_ERR("Division by zero")` if trying to divides by 0
/// # Examples
/// ```
/// # use rispreter_repl::lval::lval_env::Lenv;
/// # use rispreter_repl::eval::eval_rispreter;
/// # use rispreter_repl::lval::lval_builtin::*;
/// # use rispreter_repl::lval::lval_def::*;
/// let mut builtins = Lenv::new();
/// Lbuiltin::add_builtins(&mut builtins);
///
/// let res = eval_rispreter(&mut builtins, "(/ 4 2)".to_string());
/// assert_eq!(2f64, res);
///
/// let res = eval_rispreter(&mut builtins, "(/ 3 0)".to_string());
/// assert_eq!(Lval::lval_err("Division by Zero".to_string()), res);
/// ```
fn div(lenv: &Rc<Lenv>, lval: &mut Lval) -> Lval {
    op(lenv, lval, '/')
}

/// Modulo operand of n arguments
/// # Examples
/// ```
/// # use rispreter_repl::lval::lval_env::Lenv;
/// # use rispreter_repl::eval::eval_rispreter;
/// # use rispreter_repl::lval::lval_builtin::*;
/// # use rispreter_repl::lval::lval_def::*;
/// let mut builtins = Lenv::new();
/// Lbuiltin::add_builtins(&mut builtins);
///
/// let res = eval_rispreter(&mut builtins, "(% 13 7 5)".to_string());
/// assert_eq!(1f64, res);
/// ```
fn modl(lenv: &Rc<Lenv>, lval: &mut Lval) -> Lval {
    op(lenv, lval, '%')
}

/// Common fn for arith functions
fn op(_lenv: &Rc<Lenv>, lval: &mut Lval, op: char) -> Lval {
    let mut x = lval.lval_pop();
    let iter = lval.cell.clone();
    for i in iter.iter() {
        let y = lval.lval_pop();
        if let LvalType::LVAL_NUM(ref mut xn) = x.ltype {
            if let LvalType::LVAL_NUM(yn) = y.ltype {
                match op {
                    '+' => {
                        *xn += yn;
                    }
                    '-' => {
                        *xn -= yn;
                    }
                    '*' => {
                        *xn *= yn;
                    }
                    '/' => {
                        if yn == 0.0 {
                            return Lval::lval_err(Lerror::DivisionByZero);
                        } else {
                            *xn /= yn;
                        }
                    }
                    '%' => {
                        *xn = (*xn as i64 % yn as i64) as f64;
                    }
                    _ => {}
                }
            } else {
                return Lval::lval_err(Lerror::WrongType {lval: i.clone(), expect: LvalTypeMeta::LvalNum, got: Box::new(y.ltype)});
            }
        } else {
            return Lval::lval_err(Lerror::WrongType {lval: i.clone(), expect: LvalTypeMeta::LvalNum, got: Box::new(x.ltype)});
        }
    }
    x
}

fn eq(_lenv: &Rc<Lenv>, lval: &mut Lval) -> Lval {
    let x = lval.lval_pop();
    let y = lval.lval_pop();
    if x == y {
        Lval::lval_bool(true)
    } else {
        Lval::lval_bool(false)
    }
}

fn neq(_lenv: &Rc<Lenv>, lval: &mut Lval) -> Lval {
    let x = lval.lval_pop();
    let y = lval.lval_pop();
    if x != y {
        Lval::lval_bool(true)
    } else {
        Lval::lval_bool(false)
    }
}

fn gt(lenv: &Rc<Lenv>, lval: &mut Lval) -> Lval {
    let x = lval.lval_pop();
    let y = lval.lval_pop();
    ord_op(lenv, x, y, "gt")
}

fn lt(lenv: &Rc<Lenv>, lval: &mut Lval) -> Lval {
    let x = lval.lval_pop();
    let y = lval.lval_pop();
    ord_op(lenv, x, y, "lt")
}

fn gte(lenv: &Rc<Lenv>, lval: &mut Lval) -> Lval {
    let x = lval.lval_pop();
    let y = lval.lval_pop();
    ord_op(lenv, x, y, "gte")
}

fn lte(lenv: &Rc<Lenv>, lval: &mut Lval) -> Lval {
    let x = lval.lval_pop();
    let y = lval.lval_pop();
    ord_op(lenv, x, y, "lte")
}

fn ord_op(_lenv: &Rc<Lenv>, x: Lval, y: Lval, op: &str) -> Lval {
    match (x.ltype, y.ltype) {
        (LvalType::LVAL_NUM(a), LvalType::LVAL_NUM(b)) => match op {
            "gt" => {
                if a > b {
                    Lval::lval_bool(true)
                } else {
                    Lval::lval_bool(false)
                }
            }
            "lt" => {
                if a < b {
                    Lval::lval_bool(true)
                } else {
                    Lval::lval_bool(false)
                }
            }
            "gte" => {
                if a >= b {
                    Lval::lval_bool(true)
                } else {
                    Lval::lval_bool(false)
                }
            }
            "lte" => {
                if a <= b {
                    Lval::lval_bool(true)
                } else {
                    Lval::lval_bool(false)
                }
            }
            _ => Lval::lval_err(Lerror::InvalidOperand {op: op.to_owned()}),
        },
        (LvalType::LVAL_STRING(a), LvalType::LVAL_STRING(b)) => match op {
            "gt" => {
                if a > b {
                    Lval::lval_bool(true)
                } else {
                    Lval::lval_bool(false)
                }
            }
            "lt" => {
                if a < b {
                    Lval::lval_bool(true)
                } else {
                    Lval::lval_bool(false)
                }
            }
            "gte" => {
                if a >= b {
                    Lval::lval_bool(true)
                } else {
                    Lval::lval_bool(false)
                }
            }
            "lte" => {
                if a <= b {
                    Lval::lval_bool(true)
                } else {
                    Lval::lval_bool(false)
                }
            }
            _ => Lval::lval_err(Lerror::InvalidOperand {op: op.to_owned()}),
        },
        (LvalType::LVAL_QEXPR, LvalType::LVAL_QEXPR) => match op {
            "gt" => {
                if x.cell.len() > y.cell.len() {
                    Lval::lval_bool(true)
                } else {
                    Lval::lval_bool(false)
                }
            }
            "lt" => {
                if x.cell.len() < y.cell.len() {
                    Lval::lval_bool(true)
                } else {
                    Lval::lval_bool(false)
                }
            }
            "gte" => {
                if x.cell.len() >= y.cell.len() {
                    Lval::lval_bool(true)
                } else {
                    Lval::lval_bool(false)
                }
            }
            "lte" => {
                if x.cell.len() <= y.cell.len() {
                    Lval::lval_bool(true)
                } else {
                    Lval::lval_bool(false)
                }
            }
            _ => Lval::lval_err(Lerror::InvalidOperand {op: op.to_owned()}),
        },
        (a,b) => Lval::lval_err(Lerror::CantCompare {left: Box::new(a), right: Box::new(b)}),
    }
}

fn lif(lenv: &Rc<Lenv>, lval: &mut Lval) -> Lval {
    lval.cell[1].ltype = LvalType::LVAL_SEXPR;
    lval.cell[2].ltype = LvalType::LVAL_SEXPR;

    match &lval.cell[0].ltype {
        LvalType::LVAL_BOOL(b) => match b {
            true => lval_eval::lval_eval(lenv, &mut lval.lval_pop_with_index(1)),
            false => lval_eval::lval_eval(lenv, &mut lval.lval_pop_with_index(2)),
        },
        LvalType::LVAL_NUM(n) => {
            if n.clone() == 0.0 {
                lval_eval::lval_eval(lenv, &mut lval.lval_pop_with_index(1))
            } else {
                lval_eval::lval_eval(lenv, &mut lval.lval_pop_with_index(2))
            }
        }
        t => Lval::lval_err(Lerror::FirstArgumentDoesNotEvalTo { expect: LvalTypeMeta::LvalBool, got: Box::new(t.clone()) }),
    }
}

/// Take the head of a Q-expression
/// # Examples
/// ```
/// # use rispreter_repl::lval::lval_env::Lenv;
/// # use rispreter_repl::eval::eval_rispreter;
/// # use rispreter_repl::lval::lval_builtin::*;
/// # use rispreter_repl::lval::lval_def::*;
/// let mut builtins = Lenv::new();
/// Lbuiltin::add_builtins(&mut builtins);
///
/// let res = eval_rispreter(&mut builtins, "(head {1 2 3})".to_string());
/// assert_eq!(1f64, *res.cell[0]);
/// ```
fn head(_lenv: &Rc<Lenv>, lval: &mut Lval) -> Lval {
    if lval.cell.len() > 1 {
        return Lval::lval_err(Lerror::WrongNumberOfArgs {lval: Box::new(lval.clone()), expect: 1, got: lval.cell.len()})
    }

    let mut qexpr = lval.lval_pop();
    if qexpr.ltype != LvalType::LVAL_QEXPR {
        return Lval::lval_err(Lerror::WrongType {lval: Box::new(qexpr.clone()), expect: LvalTypeMeta::LvalQexpr , got: Box::new(qexpr.ltype)})
    }

    if qexpr.cell.len() == 0 {
        return Lval::lval_err(Lerror::EmptyList {lval: Box::new(qexpr)})
    }

    let mut head = Lval::lval_qexpr();
    head.add_cell(qexpr.lval_pop());
    head
}

/// Take the tail (all but first) of a Q-expression
/// # Examples
/// ```
/// # use rispreter_repl::lval::lval_env::Lenv;
/// # use rispreter_repl::eval::eval_rispreter;
/// # use rispreter_repl::lval::lval_builtin::*;
/// # use rispreter_repl::lval::lval_def::*;
/// let mut builtins = Lenv::new();
/// Lbuiltin::add_builtins(&mut builtins);
///
/// let head_of_tail = eval_rispreter(&mut builtins, "(head (tail {1 2 3}))".to_string());
/// assert_eq!(2f64, *head_of_tail.cell[0]);
/// ```
fn tail(_env: &Rc<Lenv>, lval: &mut Lval) -> Lval {
    if lval.cell.len() > 1 {
        return Lval::lval_err(Lerror::WrongNumberOfArgs {lval: Box::new(lval.clone()), expect: 1, got: lval.cell.len()})
    }

    let mut qexpr = lval.lval_pop();
    if qexpr.ltype != LvalType::LVAL_QEXPR {
        return Lval::lval_err(Lerror::WrongType {lval: Box::new(qexpr.clone()), expect: LvalTypeMeta::LvalQexpr , got: Box::new(qexpr.ltype)})
    }

    if qexpr.cell.len() == 0 {
        return Lval::lval_err(Lerror::EmptyList {lval: Box::new(qexpr)})
    }

    let tail = qexpr.lval_split(1);
    tail
}

/// Transform all following arguments in a Q-expression
/// # Examples
/// ```
/// # use rispreter_repl::lval::lval_env::Lenv;
/// # use rispreter_repl::eval::eval_rispreter;
/// # use rispreter_repl::lval::lval_builtin::*;
/// # use rispreter_repl::lval::lval_def::*;
/// let mut builtins = Lenv::new();
/// Lbuiltin::add_builtins(&mut builtins);
///
/// let res = eval_rispreter(&mut builtins, "(head (list 1 2 3))".to_string());
/// assert_eq!(1f64, *res.cell[0]);
/// ```
pub fn list(_env: &Rc<Lenv>, lval: &mut Lval) -> Lval {
    lval.ltype = LvalType::LVAL_QEXPR;
    lval.clone()
}

/// Joins two Q-expressions, where the first is joined in the left side of the second
/// # Examples
/// ```
/// # use rispreter_repl::lval::lval_env::Lenv;
/// # use rispreter_repl::eval::eval_rispreter;
/// # use rispreter_repl::lval::lval_builtin::*;
/// # use rispreter_repl::lval::lval_def::*;
/// let mut builtins = Lenv::new();
/// Lbuiltin::add_builtins(&mut builtins);
///
/// let res = eval_rispreter(&mut builtins, "(head (join {1} {2 3}))".to_string());
/// assert_eq!(1f64, *res.cell[0]);
/// ```
fn join(_env: &Rc<Lenv>, lval: &mut Lval) -> Lval {
    if lval.cell.len() != 2 {
        return Lval::lval_err(Lerror::WrongNumberOfArgs {lval: Box::new(lval.clone()), expect: 2, got: lval.cell.len()})
    }
    if lval.cell[0].ltype != LvalType::LVAL_QEXPR {
        return Lval::lval_err(Lerror::WrongType {lval: lval.cell[0].clone(), expect: LvalTypeMeta::LvalQexpr , got: Box::new(lval.cell[0].ltype.clone())})
    }
    if lval.cell[1].ltype != LvalType::LVAL_QEXPR {
        return Lval::lval_err(Lerror::WrongType {lval: lval.cell[1].clone(), expect: LvalTypeMeta::LvalQexpr , got: Box::new(lval.cell[1].ltype.clone())})
    }

    let mut y = lval.lval_pop();
    if y.cell.len() == 0 {
        return Lval::lval_err(Lerror::EmptyList {lval: Box::new(y)})
    }
    let mut x = lval.lval_pop();
    if x.cell.len() == 0 {
        return Lval::lval_err(Lerror::EmptyList {lval: Box::new(x)})
    }

    y.cell.append(&mut x.cell);
    y
}

/// The classic cons, put a element in a Q-expression
/// # Examples
/// ```
/// # use rispreter_repl::lval::lval_env::Lenv;
/// # use rispreter_repl::eval::eval_rispreter;
/// # use rispreter_repl::lval::lval_builtin::*;
/// # use rispreter_repl::lval::lval_def::*;
/// let mut builtins = Lenv::new();
/// Lbuiltin::add_builtins(&mut builtins);
///
/// let res = eval_rispreter(&mut builtins, "(head (cons 1 {2 3}))".to_string());
/// assert_eq!(1f64, *res.cell[0]);
/// ```
fn cons(_env: &Rc<Lenv>, lval: &mut Lval) -> Lval {
    if lval.cell.len() != 2 {
        return Lval::lval_err(Lerror::WrongNumberOfArgs {lval: Box::new(lval.clone()), expect: 2, got: lval.cell.len()})
    }
    if lval.cell[1].ltype != LvalType::LVAL_QEXPR {
        return Lval::lval_err(Lerror::WrongType {lval: lval.cell[1].clone(), expect: LvalTypeMeta::LvalQexpr , got: Box::new(lval.cell[1].ltype.clone())})
    }
    let x = lval.lval_pop();
    let mut qexpr = lval.lval_pop();
    qexpr.cell.push_front(Box::new(x));
    qexpr
}

/// Evaluates a Q-expression, basicaly it transforms a Q-expr into a S-expr
/// # Examples
/// ```
/// # use rispreter_repl::lval::lval_env::Lenv;
/// # use rispreter_repl::eval::eval_rispreter;
/// # use rispreter_repl::lval::lval_builtin::*;
/// # use rispreter_repl::lval::lval_def::*;
/// let mut builtins = Lenv::new();
/// Lbuiltin::add_builtins(&mut builtins);
///
/// let res = eval_rispreter(&mut builtins, "(eval {+ 1 2 3})".to_string());
/// assert_eq!(6f64, res);
/// ```
pub fn eval(env: &Rc<Lenv>, lval: &mut Lval) -> Lval {
    // if lval.cell.len() == 0 || lval.cell.len() > 1 {
    //     return Lval::lval_error_argssize(lval.cell.len(), 1)
    // }
    if lval.cell[0].ltype != LvalType::LVAL_QEXPR {
        return Lval::lval_err(Lerror::WrongType {lval: lval.cell[0].clone(), expect: LvalTypeMeta::LvalQexpr , got: Box::new(lval.cell[0].ltype.clone())})
    }

    let mut x = lval.lval_take(0);
    x.ltype = LvalType::LVAL_SEXPR;
    lval_eval::lval_eval(env, &mut x)
}

fn def(env: &Rc<Lenv>, lval: &mut Lval) -> Lval {
    var(env, lval, "def")
}

/// Binds the n symbols of an Q-expression in its followings bindings.
/// # Examples
/// ```
/// # use rispreter_repl::lval::lval_env::Lenv;
/// # use rispreter_repl::eval::eval_rispreter;
/// # use rispreter_repl::lval::lval_builtin::*;
/// # use rispreter_repl::lval::lval_def::*;
/// let mut builtins = Lenv::new();
/// Lbuiltin::add_builtins(&mut builtins);
///
/// eval_rispreter(&mut builtins, "(def {x} {1 2 3})".to_string());
/// let res = eval_rispreter(&mut builtins, "(head x)".to_string());
/// assert_eq!(1f64, *res.cell[0]);
/// ```
fn put(env: &Rc<Lenv>, lval: &mut Lval) -> Lval {
    var(env, lval, "put")
}

fn var(env: &Rc<Lenv>, lval: &mut Lval, func: &str) -> Lval {

    // if let LvalType::LVAL_QEXPR = &lval.cell[0].ltype {
    // } else {
    //     return Lval::lval_err(format!(
    //         "not a Q-expression got {}, expect {}",
    //         lval.ltype,
    //         LvalType::LVAL_QEXPR
    //     ));
    // }
    // let left_len = lval.cell[0].cell.len();
    // let mut right_len = 0;
    // for _ in 1..lval.cell.len() {
    //     right_len += 1;
    // }
    // if left_len != right_len {
    //     return Lval::lval_err(format!("'def' expects a equal number of bindings. Got left: {} right: {}, expects left: {} right: {}",
    //                                 left_len, right_len, left_len, left_len));
    // }
    //
    // for cell in lval.cell[0].cell.clone() {
    //     match cell.ltype {
    //         LvalType::LVAL_SYM(_sym) => {
    //             continue;
    //         }
    //         _ => return Lval::lval_error_type(cell.ltype, LvalType::LVAL_SYM("symbol".to_string())),
    //     }
    // }
    //
    // let symbols_list = lval.lval_pop();
    //
    // for i in 0..symbols_list.cell.len() {
    //     if let LvalType::LVAL_SYM(str) = &symbols_list.cell[i].ltype {
    //         match func {
    //             "def" => {
    //                 env.def(str.to_string(), *lval.cell[i].clone()).unwrap();
    //             }
    //             "put" => {
    //                 env.put(str.to_string(), *lval.cell[i].clone()).unwrap();
    //             }
    //             _ => {}
    //         }
    //     }
    // }

    if let LvalType::LVAL_QEXPR = &lval.cell[0].ltype {
    } else {
        return Lval::lval_err(Lerror::WrongType {lval: Box::new(lval.clone()), expect: LvalTypeMeta::LvalQexpr , got: Box::new(lval.ltype.clone())})
    }
    let syms = &lval.cell[0];
    for i in 0..syms.cell.len() {
        if let LvalType::LVAL_SYM(_s) = &syms.cell[i].ltype {
        } else {
            return Lval::lval_err(Lerror::WrongType {lval: lval.cell[i].clone(), expect: LvalTypeMeta::LvalSym , got: Box::new(lval.cell[i].ltype.clone())})
        }
    }
    if !(syms.cell.len() == lval.cell.len()-1) {
        return Lval::lval_err(Lerror::IncompatibleNumberOfArgs{ lval_left: syms.clone(), expect_left: syms.cell.len(), expect_right: syms.cell.len(), lval_right: Box::new(lval.clone()), got_left: syms.cell.len(), got_right: lval.cell.len()-1});
    }

    for i in 0..syms.cell.len() {
        if let LvalType::LVAL_SYM(str) = &syms.cell[i].ltype {
            match func {
                "def" => {
                    env.def(str.to_string(), *lval.cell[i+1].clone()).unwrap();
                }
                "put" => {
                    env.put(str.to_string(), *lval.cell[i+1].clone()).unwrap();
                }
                _ => {}
            }
        }
    }
    Lval::lval_sexpr()
}

fn lambda(_env: &Rc<Lenv>, lval: &mut Lval) -> Lval {
    let formals = lval.lval_pop();
    let body = lval.lval_pop();

    Lval::lval_lambda(formals, body)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// tests the expression (+ 1 2 3)
    fn lbuiltin_op_add() {
        let mut lenv = Lenv::new();
        let mut top = Lval::lval_sexpr();
        let plus_op = Lval::lval_sym("+".to_string());
        let one = Lval::lval_num(1.0);
        let two = Lval::lval_num(2.0);
        let three = Lval::lval_num(3.0);
        top.add_cell(plus_op)
            .add_cell(one)
            .add_cell(two)
            .add_cell(three);

        assert_eq!(top.cell.len(), 4);
        top.lval_pop();
        top = op(&mut lenv, &mut top, '+');
        println!("{:?}", top);
        assert_eq!(top.cell.len(), 0);
        assert_eq!(top.ltype, LvalType::LVAL_NUM(6.0));
    }

    #[test]
    /// tests the expression (- 4 2 1)
    fn lbuiltin_op_sub() {
        let mut lenv = Lenv::new();
        let mut top = Lval::lval_sexpr();
        let sub_op = Lval::lval_sym("-".to_string());
        let one = Lval::lval_num(4.0);
        let two = Lval::lval_num(2.0);
        let three = Lval::lval_num(1.0);
        top.add_cell(sub_op)
            .add_cell(one)
            .add_cell(two)
            .add_cell(three);

        assert_eq!(top.cell.len(), 4);
        top.lval_pop();
        top = op(&mut lenv, &mut top, '-');
        println!("{:?}", top);
        assert_eq!(top.cell.len(), 0);
        assert_eq!(top.ltype, LvalType::LVAL_NUM(1.0));
    }

    #[test]
    /// tests the expression (* 1 2 4)
    fn lbuiltin_op_mul() {
        let mut lenv = Lenv::new();
        let mut top = Lval::lval_sexpr();
        let mult_op = Lval::lval_sym("*".to_string());
        let one = Lval::lval_num(1.0);
        let two = Lval::lval_num(2.0);
        let three = Lval::lval_num(4.0);
        top.add_cell(mult_op)
            .add_cell(one)
            .add_cell(two)
            .add_cell(three);

        assert_eq!(top.cell.len(), 4);
        top.lval_pop();
        top = op(&mut lenv, &mut top, '*');
        println!("{:?}", top);
        assert_eq!(top.cell.len(), 0);
        assert_eq!(top.ltype, LvalType::LVAL_NUM(8.0));
    }

    #[test]
    /// tests the expression (/ 9 5 2)
    fn lbuiltin_op_div() {
        let mut lenv = Lenv::new();
        let mut top = Lval::lval_sexpr();
        let div_op = Lval::lval_sym("/".to_string());
        let one = Lval::lval_num(9.0);
        let two = Lval::lval_num(5.0);
        let three = Lval::lval_num(2.0);
        top.add_cell(div_op)
            .add_cell(one)
            .add_cell(two)
            .add_cell(three);

        assert_eq!(top.cell.len(), 4);
        top.lval_pop();
        top = op(&mut lenv, &mut top, '/');
        println!("{:?}", top);
        assert_eq!(top.cell.len(), 0);
        assert_eq!(top.ltype, LvalType::LVAL_NUM(0.9));
    }

    #[test]
    fn lbuiltin_head() {
        let mut lenv = Lenv::new();
        let mut top = Lval::lval_sexpr();
        let head_op = Lval::lval_sym("head".to_string());
        let mut qexpr = Lval::lval_qexpr();
        let a = Lval::lval_num(1.0);
        let b = Lval::lval_num(2.0);
        let c = Lval::lval_num(3.0);
        qexpr.add_cell(a).add_cell(b).add_cell(c);
        top.add_cell(head_op).add_cell(qexpr);
        top.lval_pop();
        top = head(&mut lenv, &mut top);
        println!("{}", top);
        assert_eq!(top.cell[0].ltype, LvalType::LVAL_NUM(1.0));
    }

    #[test]
    fn lbuiltin_tail() {
        let mut lenv = Lenv::new();
        let mut top = Lval::lval_sexpr();
        let head_op = Lval::lval_sym("head".to_string());
        let mut qexpr = Lval::lval_qexpr();
        let a = Lval::lval_num(1.0);
        let b = Lval::lval_num(2.0);
        let c = Lval::lval_num(3.0);
        qexpr.add_cell(a).add_cell(b).add_cell(c);
        top.add_cell(head_op).add_cell(qexpr);
        top.lval_pop();
        top = tail(&mut lenv, &mut top);
        assert_eq!(top.ltype, LvalType::LVAL_QEXPR);
        assert_eq!(top.cell[0].ltype, LvalType::LVAL_NUM(2.0));
        assert_eq!(top.cell[1].ltype, LvalType::LVAL_NUM(3.0));
    }

    #[test]
    fn lbuiltin_cons() {
        let mut lenv = Lenv::new();
        let mut top = Lval::lval_sexpr();
        let head_op = Lval::lval_sym("cons".to_string());
        let to_be_consed = Lval::lval_num(1.0);

        let mut qexpr = Lval::lval_qexpr();
        let a = Lval::lval_num(2.0);
        let b = Lval::lval_num(3.0);
        qexpr.add_cell(a).add_cell(b);

        top.add_cell(head_op).add_cell(to_be_consed).add_cell(qexpr);
        top.lval_pop();
        top = cons(&mut lenv, &mut top);
        assert_eq!(top.ltype, LvalType::LVAL_QEXPR);
        assert_eq!(top.cell[0].ltype, LvalType::LVAL_NUM(1.0));
        assert_eq!(top.cell[1].ltype, LvalType::LVAL_NUM(2.0));
        assert_eq!(top.cell[2].ltype, LvalType::LVAL_NUM(3.0));
    }

    #[test]
    fn lbuiltin_list() {
        let mut lenv = Lenv::new();
        let mut top = Lval::lval_sexpr();
        let one = Lval::lval_num(1.0);
        let two = Lval::lval_num(2.0);
        let three = Lval::lval_num(3.0);
        top.add_cell(one).add_cell(two).add_cell(three);
        assert_eq!(top.ltype, LvalType::LVAL_SEXPR);
        println!("{:?}", top);
        top = list(&mut lenv, &mut top);
        println!("{:?}", top);
        assert_eq!(top.ltype, LvalType::LVAL_QEXPR);
        assert_eq!(top.cell.len(), 3);
    }

    #[test]
    fn lbuiltin_join() {
        let mut lenv = Lenv::new();
        let mut top = Lval::lval_sexpr();

        let mut one = Lval::lval_qexpr();
        let one_one = Lval::lval_num(1.0);
        let one_two = Lval::lval_num(2.0);
        one.add_cell(one_one).add_cell(one_two);

        let mut two = Lval::lval_qexpr();
        let two_one = Lval::lval_num(3.0);
        let two_two = Lval::lval_num(4.0);
        two.add_cell(two_one).add_cell(two_two);

        top.add_cell(one).add_cell(two);

        println!("{:?}", top);
        top = join(&mut lenv, &mut top);
        println!("{:?}", top);
        assert_eq!(top.ltype, LvalType::LVAL_QEXPR);
        assert_eq!(top.cell.len(), 4);
        assert_eq!(top.cell[0].ltype, LvalType::LVAL_NUM(1.0));
        assert_eq!(top.cell[3].ltype, LvalType::LVAL_NUM(4.0));
    }

    #[test]
    fn lbuiltin_eval() {
        let mut lenv = Lenv::new();
        Lbuiltin::add_builtins(&mut lenv);
        let mut top = Lval::lval_sexpr();

        let mut sub = Lval::lval_qexpr();
        let one = Lval::lval_sym("+".to_string());
        let two = Lval::lval_num(1.0);
        let three = Lval::lval_num(2.0);

        sub.add_cell(one).add_cell(two).add_cell(three);
        top.add_cell(sub);
        top = eval(&mut lenv, &mut top);
        assert_eq!(top.ltype, LvalType::LVAL_NUM(3.0));
    }
}
