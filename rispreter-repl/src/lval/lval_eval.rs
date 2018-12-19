use crate::lval::lval_def::*;

pub fn lval_eval(lenv: &mut Lenv, lval: &mut Lval) -> Lval {
    match &lval.ltype {
        LvalType::LVAL_SYM(sym) => {
            let x = lenv.get(&sym);
            match x {
                Some(v) => {
                    return *v.clone();
                },
                None => {
                    return Lval::lval_err(format!("Can't find {:?}", sym));
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

pub fn lval_eval_sexpr(lenv: &mut Lenv, lval: &mut Lval) -> Lval {

    for i in 0..lval.cell.len() {
        lval.cell[i] = Box::new(lval_eval(lenv, &mut lval.cell[i]));
    }


    for i in 0..lval.cell.len() {
        if let LvalType::LVAL_ERR(_err) = &lval.cell[i].ltype {
            return lval.lval_take(i)
        } else {
            continue;
        }

        // match lval.cell[i].ltype  {
        //     LvalType::LVAL_ERR(_err) => {
        //         return lval.lval_take(i)
        //     },
        //     _ => {
        //         continue;
        //     }
        // }
    }

    if lval.cell.len() == 0 { return lval.clone(); }
    if lval.cell.len() == 1 { return lval.lval_take(0); }

    let f = lval.lval_pop();
    if let LvalType::LVAL_FUN(fun) = &f.ltype {
        fun.clone().0(lenv, lval)
    } else {
        Lval::lval_err("First element is not a function!".to_string())
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::lval::lval_builtin::*;
    // use crate::lval_def::*;

    #[test]
    ///     risp> 1.0
    ///     1.0
    ///     risp> ()
    ///     ()
    ///     risp> symbol
    ///     symbol
    fn test_lval_single() {
        let mut lval = Lval::lval_num(1.0);
        let mut lenv = Lenv::new();
        let result = lval_eval(&mut lenv, &mut lval);
        assert_eq!(result, Lval::lval_num(1.0));

        let mut lval = Lval::lval_sexpr();
        let mut lenv = Lenv::new();
        let result = lval_eval(&mut lenv, &mut lval);
        assert_eq!(result, Lval::lval_sexpr());
    }

    #[test]
    ///     risp> (+ 1 1)
    ///     2
    fn test_lval_sexpr() {
        let mut env = Lenv::new();
        Lbuiltin::add_builtins(&mut env);

        let mut top = Lval::lval_sexpr();
        let first = Lval::lval_sym("+".to_string());
        let second = Lval::lval_num(1.0);
        let third = Lval::lval_num(1.0);
        top.add_cell(first).add_cell(second).add_cell(third);
        let res = lval_eval(&mut env, &mut top);
        assert_eq!(res.ltype, LvalType::LVAL_NUM(2.0));
    }

    #[test]
    ///     risp> (+ 1 (+ 2 3))
    ///     6
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
        let res = lval_eval(&mut env, &mut top);
        assert_eq!(res.ltype, LvalType::LVAL_NUM(6.0));
    }

    #[test]
    ///     risp> (head {1 2 3})
    ///     1
    fn head_expr() {
        let mut env = Lenv::new();
        Lbuiltin::add_builtins(&mut env);

        let mut top = Lval::lval_sexpr();
        let head = Lval::lval_sym("head".to_string());
        let mut qexpr = Lval::lval_qexpr();
        let a = Lval::lval_num(1.0);
        let b = Lval::lval_num(2.0);
        let c = Lval::lval_num(3.0);
        qexpr.add_cell(a).add_cell(b).add_cell(c);
        top.add_cell(head).add_cell(qexpr);
        let res = lval_eval(&mut env, &mut top);
        assert_eq!(res.ltype, LvalType::LVAL_NUM(1.0));
    }
}
