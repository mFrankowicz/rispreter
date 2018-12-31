use crate::lval::lval_builtin::Lbuiltin;
use crate::lval::lval_def::Lval;
use crate::lval::lval_error::Lerror;
use fnv::FnvHashMap;
use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Default, Debug)]
pub struct Lenv {
    parent: Option<Parent>,
    vals: RefCell<FnvHashMap<String, Lval>>,
    deepness: RefCell<usize>,
}

impl Lenv {
    pub fn new() -> Rc<Lenv> {
        Lenv::init(None)
    }

    pub fn add_builtin(&self, key: &str, b: Lbuiltin) {
        let mut vals = self.vals.borrow_mut();
        vals.insert(key.to_string(), Lval::lval_fun(b));
    }

    pub fn from(parent: &Rc<Lenv>) -> Rc<Lenv> {
        Lenv::init(Some(Parent::Strong(Rc::clone(parent))))
    }

    pub fn from_weak(parent: &Rc<Lenv>) -> Rc<Lenv> {
        if parent.has_weak() {
            return Lenv::from(parent);
        }
        Lenv::init(Some(Parent::Weak(Rc::downgrade(parent))))
    }

    pub fn put(&self, id: String, val: Lval) -> Result<(), String> {
        let mut vals = self.vals.borrow_mut();
        vals.insert(id, val);
        Ok(())
    }

    pub fn def(&self, id: String, val: Lval) -> Result<(), String> {
        if let Some(ref parent) = self.parent {
            return parent.def(id, val);
        } else {
            self.put(id, val)
        }
    }

    pub fn get(&self, id: String) -> Result<Lval, String> {
        //println!("trying to get {}", id);
        let vals = self.vals.borrow();
        if let Some(val) = vals.get(&id) {
            //println!("got {}", val);
            Ok(val.clone())
        } else {
            //println!("{} not binded in this env: \n {:?} \n", id, self);
            //println!("checking on parent");
            if let Some(ref parent) = self.parent {
                //println!("this has parent");
                parent.get(id)
            } else {
                Ok(Lval::lval_err(Lerror::SymbolNotBinded { sym: id }))
            }
        }
    }

    pub fn assign_at(&self, id: String, val: Lval, dist: Option<&usize>) -> Result<Lval, String> {
        if dist.map_or(0, |d| *d) == 0 {
            return self.assign(id, val);
        }

        let d: usize = *dist.unwrap();

        if let Some(ancestor) = self.ancestor(d) {
            return ancestor.assign(id, val);
        }

        Err(format!("ancestor is undefined at depth {}", d))
    }

    pub fn get_at(&self, id: String, dist: Option<&usize>) -> Result<Lval, String> {
        if dist.is_none() {
            return self.get_global(id);
        }

        let d = *dist.unwrap();

        if d == 0 {
            return self.get(id);
        }

        if let Some(ancestor) = self.ancestor(d) {
            return ancestor.get(id);
        }

        Err(format!("ancestor is undefined at depth {}", d))
    }

    pub fn has_weak(&self) -> bool {
        match self.parent {
            Some(ref p) => p.has_weak(),
            None => false,
        }
    }
}

impl Lenv {
    fn init(parent: Option<Parent>) -> Rc<Lenv> {
        Rc::new(Lenv {
            parent,
            vals: RefCell::new(FnvHashMap::default()),
            deepness: RefCell::new(0),
        })
    }

    fn ancestor(&self, dist: usize) -> Option<Parent> {
        let mut env = self.parent.clone();

        for _ in 1..dist {
            env = env?.parent();
        }

        env
    }

    fn assign(&self, id: String, val: Lval) -> Result<Lval, String> {
        //let name = &id.lexeme;
        let mut vals = self.vals.borrow_mut();

        if !vals.contains_key(&id) {
            if let Some(ref parent) = self.parent {
                return parent.assign(id, val);
            }

            return Err(format!("variable `{}` is undefined", id));
        }

        let _ = vals.insert(id.to_owned(), val.clone());
        Ok(val)
    }

    fn get_global(&self, id: String) -> Result<Lval, String> {
        match self.parent {
            None => self.get(id),
            Some(ref parent) => parent.get_global(id),
        }
    }
}

// impl Drop for Lenv {
//     fn drop(&mut self) {
//         let details = match self.parent {
//             Some(ref p) => match *p {
//                 Parent::Strong(ref e) => format!(
//                     "Lenv::Strong (parent now has {} refs)",
//                     e.parent.as_ref().map_or(0, |p| p.refs()-1)),
//                 Parent::Weak(ref w) => match w.upgrade() {
//                     Some(ref e) => format!(
//                         "Lenv::Weak (parent has {} refs)",
//                         e.parent.as_ref().map_or(0, |p| p.refs())),
//                     None => "Lenv::Unknown (parent dropped out of scope)".to_owned(),
//                 }
//             },
//             None => "Lenv::Root".to_owned(),
//         };
//
//         println!("{} with keys {:?}", details, self.vals.borrow().keys());
//     }
// }

#[derive(Debug, Clone)]
enum Parent {
    Strong(Rc<Lenv>),
    Weak(Weak<Lenv>),
}

macro_rules! parent_call {
    ($self:ident$(.$member:ident)+ $(, $arg:expr)* ) => {
        match *$self {
            Parent::Strong(ref e) => e$(.$member)+($($arg,)*),
            Parent::Weak(ref w) => match w.upgrade() {
                Some(ref e) => e$(.$member)+($($arg,)*),
                None => panic!("parent env went out of scope"),
            }
        }
    };
}

impl Parent {
    fn parent(&self) -> Option<Parent> {
        parent_call!(self.parent.clone)
    }
    fn assign(&self, id: String, val: Lval) -> Result<Lval, String> {
        parent_call!(self.assign, id, val)
    }
    fn get(&self, id: String) -> Result<Lval, String> {
        parent_call!(self.get, id)
    }
    fn get_global(&self, id: String) -> Result<Lval, String> {
        parent_call!(self.get_global, id)
    }
    fn def(&self, id: String, val: Lval) -> Result<(), String> {
        parent_call!(self.def, id, val)
    }
    // fn refs(&self) -> usize {
    //     match *self {
    //         Parent::Strong(ref e) => Rc::strong_count(e),
    //         Parent::Weak(ref w) => match w.upgrade() {
    //             Some(ref e) => Rc::strong_count(e) - 1,
    //             None => 0,
    //         }
    //     }
    // }

    fn has_weak(&self) -> bool {
        match *self {
            Parent::Strong(ref e) => e.has_weak(),
            Parent::Weak(_) => true,
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     fn recursive_call(env: &Rc<Lenv>) -> i32{
//         let v = env.get("val".to_owned()).unwrap();
//         if v == 0 {
//             return 0;
//         } else {
//             let child = Lenv::from(&env);
//             child.assign("val".to_owned(), v -1).unwrap();
//             recursive_call(&child)
//         }
//     }
//
//     #[test]
//     fn env(){
//         let env = Lenv::new();
//         env.define("x".to_owned(), 1).unwrap();
//         assert_eq!(Ok(1), env.get("x".to_owned()));
//         env.assign("x".to_owned(), 2).unwrap();
//         assert_eq!(Ok(2), env.get("x".to_owned()));
//     }
//
//     #[test]
//     fn env_parent() {
//         let paren = Lenv::new();
//         let child = Lenv::from(&paren);
//         paren.define("x".to_owned(), 1).unwrap();
//         child.define("y".to_owned(), 2).unwrap();
//         assert_eq!(Ok(1), child.get("x".to_owned()));
//         assert_eq!(Err("variable `y` is undefined".to_owned()), paren.get("y".to_owned()));
//     }
//
//     #[test]
//     fn env_recursive_call() {
//         let env = Lenv::new();
//         env.define("val".to_owned(), 10).unwrap();
//         let res = recursive_call(&env);
//         assert_eq!(0, res);
//     }
// }
