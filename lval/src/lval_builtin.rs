use crate::lval_def::*;
use crate::lval_eval;

pub struct Lbuiltin(pub fn(lenv: &Lenv, lval: &mut Lval) -> Lval);

impl Lbuiltin {

    pub fn add_builtins(lenv: &mut Lenv) {
        lenv.add_builtin("+", Lbuiltin::lbuiltin_add());
        lenv.add_builtin("-", Lbuiltin::lbuiltin_sub());
        lenv.add_builtin("*", Lbuiltin::lbuiltin_mul());
        lenv.add_builtin("/", Lbuiltin::lbuiltin_div());
        lenv.add_builtin("%", Lbuiltin::lbuiltin_mod());
        lenv.add_builtin("head", Lbuiltin::lbuiltin_head());
        lenv.add_builtin("tail", Lbuiltin::lbuiltin_tail());
        lenv.add_builtin("list", Lbuiltin::lbuiltin_list());
        lenv.add_builtin("join", Lbuiltin::lbuiltin_join());
        lenv.add_builtin("cons", Lbuiltin::lbuiltin_cons());
        lenv.add_builtin("eval", Lbuiltin::lbuiltin_eval());

    }

    fn lbuiltin_add() -> Lbuiltin {
        Lbuiltin(add)
    }

    fn lbuiltin_sub() -> Lbuiltin {
        Lbuiltin(sub)
    }

    fn lbuiltin_mul() -> Lbuiltin {
        Lbuiltin(mul)
    }

    fn lbuiltin_div() -> Lbuiltin {
        Lbuiltin(div)
    }

    fn lbuiltin_mod() -> Lbuiltin {
        Lbuiltin(modl)
    }

    fn lbuiltin_head() -> Lbuiltin {
        Lbuiltin(head)
    }

    fn lbuiltin_tail() -> Lbuiltin {
        Lbuiltin(tail)
    }

    fn lbuiltin_list() -> Lbuiltin {
        Lbuiltin(list)
    }

    fn lbuiltin_join() -> Lbuiltin {
        Lbuiltin(join)
    }

    fn lbuiltin_cons() -> Lbuiltin {
        Lbuiltin(cons)
    }

    fn lbuiltin_eval() -> Lbuiltin {
        Lbuiltin(eval)
    }
}

impl PartialEq for Lbuiltin {
    fn eq(&self, other: &Self) -> bool {
        self.0 as usize == other.0 as usize
    }
}

impl Clone for Lbuiltin {
    fn clone(&self) -> Self {
        Lbuiltin(self.0.clone())
    }
}

impl std::fmt::Debug for Lbuiltin {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Lbuiltin(0x{:x})", self.0 as usize)
    }
}

/// builtins

fn add(lenv: &Lenv, lval: &mut Lval,) -> Lval {
    op(lenv, lval, '+')
}

fn sub(lenv: &Lenv, lval: &mut Lval,) -> Lval {
    op(lenv, lval, '-')
}

fn mul(lenv: &Lenv, lval: &mut Lval,) -> Lval {
    op(lenv, lval, '*')
}

fn div(lenv: &Lenv, lval: &mut Lval,) -> Lval {
    op(lenv, lval, '/')
}

fn modl(lenv: &Lenv, lval: &mut Lval,) -> Lval {
    op(lenv, lval, '%')
}

// TODO: make better errors
fn op(_lenv: &Lenv, lval: &mut Lval, op: char) -> Lval {

    let mut x = lval.lval_pop();
    let iter = lval.cell.clone();
    for _i in iter.iter() {
         let y = lval.lval_pop();
        if let LvalType::LVAL_NUM(ref mut xn) = x.ltype {
            if let LvalType::LVAL_NUM(yn) = y.ltype {
                match op {
                    '+' => { *xn += yn; },
                    '-' => { *xn -= yn; },
                    '*' => { *xn *= yn; },
                    '/' => {
                        if yn == 0.0 {
                            return Lval::lval_err("Division by Zero".to_string());
                        } else {
                            *xn /= yn;
                        }
                    },
                    '%' => { *xn = (*xn as i64 % yn as i64) as f64; },
                    _ => { }
                }
            } else {
                return Lval::lval_err(format!("Can't operate in a non number! Got {:?} expect LvalType::LVAL_NUM", y.ltype));
            }
        } else {
            return Lval::lval_err(format!("Can't operate in a non number! Got {:?} expect LvalType::LVAL_NUM", x.ltype));
        }
    }
    x
}

fn head(_lenv: &Lenv, lval: &mut Lval) -> Lval {
    let mut qexpr = lval.lval_pop();
    let head = qexpr.lval_pop();
    head
}

fn tail(_env: &Lenv, lval: &mut Lval) ->  Lval {
    let mut qexpr = lval.lval_pop();
    let tail = qexpr.lval_split(1);
    tail
}

fn list(_env: &Lenv, lval: &mut Lval) -> Lval {
    lval.ltype = LvalType::LVAL_QEXPR;
    lval.clone()
}

fn join(_env: &Lenv, lval: &mut Lval) -> Lval {
    let mut y = lval.lval_pop();
    let mut x = lval.lval_pop();
    y.cell.append(&mut x.cell);
    y
}

fn cons(_env: &Lenv, lval: &mut Lval) -> Lval {
    let x = lval.lval_pop();
    let mut qexpr = lval.lval_pop();
    qexpr.cell.push_front(Box::new(x));
    qexpr
}

fn eval(env: &Lenv, lval: &mut Lval) -> Lval {
    let mut x = lval.lval_take(0);
    x.ltype = LvalType::LVAL_SEXPR;
    lval_eval::lval_eval(&env, &mut x)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// tests the expression (+ 1 2 3)
    fn lbuiltin_op_add() {
        let lenv = Lenv::new();
        let mut top = Lval::lval_sexpr();
        let plus_op = Lval::lval_sym("+".to_string());
        let one = Lval::lval_num(1.0);
        let two = Lval::lval_num(2.0);
        let three = Lval::lval_num(3.0);
        top.add_cell(plus_op).add_cell(one).add_cell(two).add_cell(three);

        assert_eq!(top.cell.len(), 4);
        top.lval_pop();
        top = op(&lenv, &mut top, '+');
        println!("{:?}", top);
        assert_eq!(top.cell.len(), 0);
        assert_eq!(top.ltype, LvalType::LVAL_NUM(6.0));
    }

    #[test]
    /// tests the expression (- 4 2 1)
    fn lbuiltin_op_sub() {
        let lenv = Lenv::new();
        let mut top = Lval::lval_sexpr();
        let sub_op = Lval::lval_sym("-".to_string());
        let one = Lval::lval_num(4.0);
        let two = Lval::lval_num(2.0);
        let three = Lval::lval_num(1.0);
        top.add_cell(sub_op).add_cell(one).add_cell(two).add_cell(three);

        assert_eq!(top.cell.len(), 4);
        top.lval_pop();
        top = op(&lenv, &mut top, '-');
        println!("{:?}", top);
        assert_eq!(top.cell.len(), 0);
        assert_eq!(top.ltype, LvalType::LVAL_NUM(1.0));
    }

    #[test]
    /// tests the expression (* 1 2 4)
    fn lbuiltin_op_mul() {
        let lenv = Lenv::new();
        let mut top = Lval::lval_sexpr();
        let mult_op = Lval::lval_sym("*".to_string());
        let one = Lval::lval_num(1.0);
        let two = Lval::lval_num(2.0);
        let three = Lval::lval_num(4.0);
        top.add_cell(mult_op).add_cell(one).add_cell(two).add_cell(three);

        assert_eq!(top.cell.len(), 4);
        top.lval_pop();
        top = op(&lenv, &mut top, '*');
        println!("{:?}", top);
        assert_eq!(top.cell.len(), 0);
        assert_eq!(top.ltype, LvalType::LVAL_NUM(8.0));
    }

    #[test]
    /// tests the expression (/ 9 5 2)
    fn lbuiltin_op_div() {
        let lenv = Lenv::new();
        let mut top = Lval::lval_sexpr();
        let div_op = Lval::lval_sym("/".to_string());
        let one = Lval::lval_num(9.0);
        let two = Lval::lval_num(5.0);
        let three = Lval::lval_num(2.0);
        top.add_cell(div_op).add_cell(one).add_cell(two).add_cell(three);

        assert_eq!(top.cell.len(), 4);
        top.lval_pop();
        top = op(&lenv, &mut top, '/');
        println!("{:?}", top);
        assert_eq!(top.cell.len(), 0);
        assert_eq!(top.ltype, LvalType::LVAL_NUM(0.9));
    }

    #[test]
    fn lbuiltin_head() {
        let lenv = Lenv::new();
        let mut top = Lval::lval_sexpr();
        let head_op = Lval::lval_sym("head".to_string());
        let mut qexpr = Lval::lval_qexpr();
        let a = Lval::lval_num(1.0);
        let b = Lval::lval_num(2.0);
        let c = Lval::lval_num(3.0);
        qexpr.add_cell(a).add_cell(b).add_cell(c);
        top.add_cell(head_op).add_cell(qexpr);
        top.lval_pop();
        top = head(&lenv, &mut top);
        assert_eq!(top.ltype, LvalType::LVAL_NUM(1.0));
    }

    #[test]
    fn lbuiltin_tail() {
        let lenv = Lenv::new();
        let mut top = Lval::lval_sexpr();
        let head_op = Lval::lval_sym("head".to_string());
        let mut qexpr = Lval::lval_qexpr();
        let a = Lval::lval_num(1.0);
        let b = Lval::lval_num(2.0);
        let c = Lval::lval_num(3.0);
        qexpr.add_cell(a).add_cell(b).add_cell(c);
        top.add_cell(head_op).add_cell(qexpr);
        top.lval_pop();
        top = tail(&lenv, &mut top);
        assert_eq!(top.ltype, LvalType::LVAL_QEXPR);
        assert_eq!(top.cell[0].ltype, LvalType::LVAL_NUM(2.0));
        assert_eq!(top.cell[1].ltype, LvalType::LVAL_NUM(3.0));
    }

    #[test]
    fn lbuiltin_cons() {
        let lenv = Lenv::new();
        let mut top = Lval::lval_sexpr();
        let head_op = Lval::lval_sym("cons".to_string());
        let to_be_consed = Lval::lval_num(1.0);

        let mut qexpr = Lval::lval_qexpr();
        let a = Lval::lval_num(2.0);
        let b = Lval::lval_num(3.0);
        qexpr.add_cell(a).add_cell(b);

        top.add_cell(head_op).add_cell(to_be_consed).add_cell(qexpr);
        top.lval_pop();
        top = cons(&lenv, &mut top);
        assert_eq!(top.ltype, LvalType::LVAL_QEXPR);
        assert_eq!(top.cell[0].ltype, LvalType::LVAL_NUM(1.0));
        assert_eq!(top.cell[1].ltype, LvalType::LVAL_NUM(2.0));
        assert_eq!(top.cell[2].ltype, LvalType::LVAL_NUM(3.0));
    }

    #[test]
    fn lbuiltin_list() {
        let lenv = Lenv::new();
        let mut top = Lval::lval_sexpr();
        let one = Lval::lval_num(1.0);
        let two = Lval::lval_num(2.0);
        let three = Lval::lval_num(3.0);
        top.add_cell(one).add_cell(two).add_cell(three);
        assert_eq!(top.ltype, LvalType::LVAL_SEXPR);
        println!("{:?}", top);
        top = list(&lenv, &mut top);
        println!("{:?}", top);
        assert_eq!(top.ltype, LvalType::LVAL_QEXPR);
        assert_eq!(top.cell.len(), 3);
    }

    #[test]
    fn lbuiltin_join() {
        let lenv = Lenv::new();
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
        top = join(&lenv, &mut top);
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
        top = eval(&lenv, &mut top);
        assert_eq!(top.ltype, LvalType::LVAL_NUM(3.0));
    }
}
