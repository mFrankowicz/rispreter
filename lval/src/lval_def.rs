use crate::lval_builtin::*;
use std::collections::HashMap;
use std::collections::VecDeque;

#[allow(non_camel_case_types)] // please
#[derive(PartialEq, Debug, Clone)]
pub enum LvalType {
    LVAL_ERR(String),
    LVAL_NUM(f64),
    LVAL_SYM(String),
    LVAL_FUN(Lbuiltin),
    LVAL_SEXPR,
    LVAL_QEXPR,
}

#[derive(PartialEq, Debug, Clone)]
pub struct Lval {
    pub ltype: LvalType,
    pub cell: VecDeque<Lval>,
}

impl Drop for Lval {
    fn drop(&mut self) {
        self.cell.clear();
    }
}

impl Lval {

    pub fn lval_num(num: f64) -> Lval {
        Lval {
            ltype: LvalType::LVAL_NUM(num),
            cell: VecDeque::new(),
        }
    }

    pub fn lval_err(err: String) -> Lval {
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

    pub fn lval_fun(func: Lbuiltin) -> Lval {
        Lval {
            ltype: LvalType::LVAL_FUN(func),
            cell: VecDeque::new(),
        }
    }

    pub fn add_cell(&mut self, lval: Lval) -> &mut Self {
        self.cell.push_back(lval);
        self
    }

    pub fn lval_pop(&mut self) -> Option<Lval> {
        if self.cell.len() > 0 {
            self.cell.pop_front()
        } else {
            None
        }
    }

    pub fn lval_take(&mut self, index: usize) -> Lval {
        let x = self.cell.remove(index).clone();
        self.cell.clear();
        x.unwrap()
    }

    pub fn lval_split(&mut self, index: usize) -> Lval {
        self.cell = self.cell.split_off(index);
        self.clone()
    }

    pub fn lval_join(&mut self, other: &mut Lval) {
        self.cell.append(&mut other.cell);
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct Lenv {
    vals: HashMap<String, Lval>,
}

impl Lenv {
    pub fn new() -> Lenv {
        Lenv {
            vals: HashMap::new(),
        }
    }

    pub fn add_builtin(&mut self, name: &str, func: Lbuiltin) {
        let lval = Lval::lval_fun(func);
        self.vals.insert(name.to_string(), lval);
    }

    pub fn get(&self, k: &String) -> Option<&Lval> {
        self.vals.get(k)
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn create_lval_type() {
        let lval = Lval::lval_err("err".to_string());
        assert_eq!(lval.ltype, LvalType::LVAL_ERR("err".to_string()));

        let lval = Lval::lval_num(1.0);
        assert_eq!(lval.ltype, LvalType::LVAL_NUM(1.0));

        let lval = Lval::lval_sym("sym".to_string());
        assert_eq!(lval.ltype, LvalType::LVAL_SYM("sym".to_string()));

        let lval = Lval::lval_sexpr();
        assert_eq!(lval.ltype, LvalType::LVAL_SEXPR);

        let lval = Lval::lval_qexpr();
        assert_eq!(lval.ltype, LvalType::LVAL_QEXPR);
    }


}
