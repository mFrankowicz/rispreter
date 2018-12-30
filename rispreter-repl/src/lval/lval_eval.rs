use crate::lval::lval_builtin;
use crate::lval::lval_def::*;
use crate::lval::lval_env::Lenv;
use std::rc::Rc;

static mut rec_count: usize = 0;

pub fn lval_eval(lenv: &Rc<Lenv>, lval: &mut Lval) -> Lval {
    match &lval.ltype {
        LvalType::LVAL_SYM(sym) => {
            if sym == "rec" {
                unsafe{
                    rec_count += 1;
                    println!("the name of this function is : {} with: {}", sym, rec_count);
                }
            }

            let x = lenv.get(sym.to_string());
            x.unwrap()
        }
        LvalType::LVAL_SEXPR => {
            return lval_eval_sexpr(lenv, lval);
        }
        _ => {
            return lval.clone();
        }
    }
}

pub fn lval_eval_sexpr(lenv: &Rc<Lenv>, lval: &mut Lval) -> Lval {
    for i in 0..lval.cell.len() {
        lval.cell[i] = Box::new(lval_eval(lenv, &mut lval.cell[i]));
    }

    for i in 0..lval.cell.len() {
        if let LvalType::LVAL_ERR(_err) = &lval.cell[i].ltype {
            return lval.lval_take(i);
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

    if lval.cell.len() == 0 {
        return lval.clone();
    }
    if lval.cell.len() == 1 {
        return lval_eval(lenv, &mut lval.lval_take(0));
    }

    let mut f = lval.lval_pop();
    lval_call(lenv, &mut f, lval)
}

pub fn lval_call(lenv: &Rc<Lenv>, f: &mut Lval, lval: &mut Lval) -> Lval {
    match f.ltype.clone() {
        // if builtin we return
        LvalType::LVAL_FUN(builtin) => builtin.clone().0(lenv, lval),
        // if we have a lambda expression then...
        LvalType::LVAL_LAMBDA(mut lambda) => {
            // record argument counts
            let given = lval.cell.len();
            let total = lambda.formals.cell.len();
            // println!("given {}", given);
            // println!("total {}", total);
            lambda.local_lenv = Lenv::from_weak(&lenv);
            // while arguments still to be processed
            while lval.cell.len() > 0 {
                // if we ran out of formal arguments to bind
                if lambda.formals.cell.len() == 0 {
                    return Lval::lval_err(format!(
                        "Function passed to many argyments. Got {}, expect {}, called from lval_call",
                        given, total
                    ));
                }

                // pop the first symbol from the formals
                // println!("lambda args formals count: {}", lambda.formals.cell.len());
                let sym = lambda.formals.lval_pop();
                if let LvalType::LVAL_SYM(s) = &sym.ltype {
                    if s == "&" {
                        //TODO: ensure its followd by another symbol
                        let next_sym = lambda.formals.lval_pop();
                        lambda
                            .local_lenv
                            .put(next_sym.to_string(), lval_builtin::list(lenv, lval)).unwrap();
                        break;
                    }
                }
                // println!("lambda args formals count after pop: {}", lambda.formals.cell.len());
                // println!("{:?}", sym.clone());

                //pop the next argument from the list
                // println!("lval args count: {}", lval.cell.len());
                let val = lval.lval_pop();
                // println!("lval args count after pop: {}", lval.cell.len());
                // println!("{:?}", val.clone());
                // bind a copy to the lambda local env
                lambda.local_lenv.put(sym.to_string(), val).unwrap();
            }

            if lambda.formals.cell.len() > 0
                && lambda.formals.cell[0].ltype == LvalType::LVAL_SYM("&".to_string())
            {
                if lambda.formals.cell.len() != 2 {
                    return Lval::lval_err(format!(
                        "Function format invalid. Symbol '&' not followed by single symbol"
                    ));
                }
                lambda.formals.lval_pop();
                let sym = lambda.formals.lval_pop();
                let val = Lval::lval_qexpr();
                lambda.local_lenv.put(sym.to_string(), val).unwrap();
            }

            // if all formals have been bound evaluate
            if lambda.formals.cell.len() == 0 {

                // apend the lambda local env to parent env
                //lambda.local_lenv.set_parent(lenv);
                // evaluetes in this new context
                return lval_builtin::eval(
                    &mut lambda.local_lenv,
                    Lval::lval_sexpr().add_cell(*lambda.body.clone()),
                );
            } else {
                // returns a partially bound evalueted lambda
                return Lval::lval_lambda_copy(lambda.local_lenv, *lambda.formals, *lambda.body);
            }
        }
        _ => Lval::lval_err(format!("Not a builtin function or a lambda")),
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::lval::lval_builtin::*;
    use crate::lval::lval_env::Lenv;
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
        third
            .add_cell(third_one)
            .add_cell(third_two)
            .add_cell(third_three);
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
        assert_eq!(res.cell[0].ltype, LvalType::LVAL_NUM(1.0));
    }
}
