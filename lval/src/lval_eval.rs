use crate::lval_def::*;

pub fn lval_eval(lenv: &Lenv, lval: &mut Lval) -> Lval {
    match lval.ltype {
        LvalType::LVAL_SYM => {
            let x = lenv.get(&lval.sym);
            match x {
                Some(v) => {
                    return v.clone();
                },
                None => {
                    return Lval::lval_err(format!("Can't find {:?}", lval.sym));
                }
            }
        },
        LvalType::LVAL_SEXPR => {
            return lval_eval_sexpr(lenv, lval);
        },
        _ => {
            return lval.clone();
        }
    }
}

pub fn lval_eval_sexpr(lenv: &Lenv, lval: &mut Lval) -> Lval {
    
    for i in 0..lval.cell.len() {
        lval.cell[i] = lval_eval(lenv, &mut lval.cell[i]);
    }

    for i in 0..lval.cell.len() {
        if lval.cell[i].ltype == LvalType::LVAL_ERR {
            return lval.lval_take(i);
        }
    }

    if lval.cell.len() == 0 { return lval.clone(); }
    if lval.cell.len() == 1 { return lval.lval_take(0); }

    let f = lval.lval_pop().unwrap();
    if f.ltype != LvalType::LVAL_FUN {
        return Lval::lval_err("First element is not a function!".to_string());
    }

    f.fun.clone().unwrap().0(lenv, lval)
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::lval_builtin::*;
    // use crate::lval_def::*;

    #[test]
    /// >>> 1.0
    /// 1.0
    /// >>> ()
    /// ()
    /// >>> symbol
    /// symbol
    fn test_lval_single() {
        let mut lval = Lval::lval_num(1.0);
        let lenv = Lenv::new();
        let result = lval_eval(&lenv, &mut lval);
        assert_eq!(result, Lval::lval_num(1.0));

        let mut lval = Lval::lval_sexpr();
        let lenv = Lenv::new();
        let result = lval_eval(&lenv, &mut lval);
        assert_eq!(result, Lval::lval_sexpr());
    }

    #[test]
    /// (+ 1 1)
    fn test_lval_sexpr() {
        let mut env = Lenv::new();
        Lbuiltin::add_builtins(&mut env);

        let mut top = Lval::lval_sexpr();
        let first = Lval::lval_sym("+".to_string());
        let second = Lval::lval_num(1.0);
        let third = Lval::lval_num(1.0);
        top.add_cell(first).add_cell(second).add_cell(third);
        let res = lval_eval(&env, &mut top);
        assert_eq!(res.num, 2.0);
    }

    #[test]
    /// (+ 1 (+ 2 3))
    fn test_lval_sexpr_with_sexpr() {
        let mut env = Lenv::new();
        Lbuiltin::add_builtins(&mut env);

        let mut top = Lval::lval_sexpr();
        let first = Lval::lval_sym("+".to_string());
        let second = Lval::lval_num(1.0);
        let mut third = Lval::lval_sexpr();
        let third_one = Lval::lval_sym("+".to_string());
        let third_two = Lval::lval_num(2.0);
        let third_three = Lval::lval_num(3.0);
        third.add_cell(third_one).add_cell(third_two).add_cell(third_three);
        top.add_cell(first).add_cell(second).add_cell(third);
        let res = lval_eval(&env, &mut top);
        assert_eq!(res.num, 6.0);
    }
}