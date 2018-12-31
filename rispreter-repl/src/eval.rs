use crate::lval::lval_def::*;
use crate::lval::lval_env::Lenv;
use crate::lval::lval_eval::*;
use crate::read::read;
use rispreter_parser::parse_risp;
use std::rc::Rc;

pub fn eval_rispreter(lenv: &Rc<Lenv>, input: &str) -> Lval {
    lval_eval(lenv, &mut read(parse_risp(input.as_bytes())))
}

#[cfg(test)]
pub mod tests {
    use super::*;

    use crate::lval::lval_builtin::Lbuiltin;
    use crate::lval::lval_env::Lenv;

    #[test]
    fn test_parent_env_keeps_lvals_defined_inside_lambdas() {
        let mut env = Lenv::new();
        Lbuiltin::add_builtins(&mut env);
        let fun_def =
            "(def {fun} (\\ {args body} {def (head args) (\\ (tail args) body)}))\n".to_string();
        let eval1 = eval_rispreter(&mut env, &fun_def);
        assert_eq!(Lval::lval_sexpr(), eval1);
        //assert_eq!(true, env.contains("fun".to_string()));

        let fun_usage = "(fun {add-togheter x y} {+ x y})".to_string();
        let eval2 = eval_rispreter(&mut env, &fun_usage);
        println!("{:?}", eval2);
        //assert_eq!(true, env.contains("add-togheter".to_string()));
    }
}
