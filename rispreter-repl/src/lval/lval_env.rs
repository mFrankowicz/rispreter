use crate::lval::lval_builtin::Lbuiltin;
use crate::lval::lval_def::Lval;
use std::collections::HashMap;
use std::ptr;

#[derive(Debug, PartialEq, Clone)]
pub struct Lenv {
    parent: *mut Lenv,
    data: Box<HashMap<String, Lval>>,
}

impl Lenv {
    pub fn new() -> Lenv {
        Lenv {
            parent: ptr::null_mut(),
            data: Box::new(HashMap::new()),
        }
    }

    pub fn add_builtin(&mut self, key: &str, builtin: Lbuiltin) {
        self.data.insert(key.to_string(), Lval::lval_fun(builtin));
    }

    pub fn put(&mut self, key: String, val: Lval) {
        self.data.insert(key, val);
    }

    pub fn def(&mut self, key: String, val: Lval) {
        if !self.parent.is_null() {
            unsafe { (*self.parent).def(key, val) }
        } else {
            self.put(key, val);
        }
    }

    pub fn get(&self, key: String) -> Lval {
        if let Some(val) = self.data.get(&key) {
            val.clone()
        } else {
            if !self.parent.is_null() {
                unsafe { (*self.parent).get(key) }
            } else {
                Lval::lval_err(format!("Can't find {}", key))
            }
        }
    }

    pub fn set_parent(&mut self, parent: &mut Lenv) {
        self.parent = parent as *mut Lenv;
    }

    pub fn contains(&self, key: String) -> bool {
        self.data.contains_key(&key)
    }
}

// impl Drop for Lenv {
//     fn drop(&mut self) {
//         self.parent = ptr::null_mut()
//     }
// }

// #[cfg(test)]
// pub mod tests {
//     use super::*;
//
//     fn rec(env: &mut Lenv<&str, i32>, mut val: i32) -> Option<i32> {
//         if val == 0 {
//             env.get("final")
//         } else {
//             let mut local_env: Lenv<&str, i32> = Lenv::new();
//             local_env.set_parent(env);
//             val -= 1;
//             rec(&mut local_env, val)
//         }
//     }
//
//     fn rec_local_def(env: &mut Lenv<&str, i32>, mut val: i32) -> Option<i32> {
//         if val == 0 {
//             env.get("local_final")
//         } else {
//             let mut local_env: Lenv<&str, i32> = Lenv::new();
//             local_env.set_parent(env);
//             local_env.def("local_final", val + 10);
//             val -= 1;
//             rec_local_def(&mut local_env, val)
//         }
//     }
//
//     #[test]
//     fn test_env() {
//         let mut env: Lenv<&str, i32> = Lenv::new();
//         let mut child: Lenv<&str, i32> = Lenv::new();
//         child.set_parent(&mut env);
//         env.put("Key", 1);
//         env.def("Jey", 2);
//         child.def("Xey", 3);
//
//         assert_eq!(Some(1), child.get("Key"));
//         assert_eq!(Some(2), child.get("Jey"));
//         assert_eq!(Some(3), child.get("Xey"));
//     }
//
//     #[test]
//     fn test_env_recursive() {
//         let mut env: Lenv<&str, i32> = Lenv::new();
//         env.put("final", 5);
//         assert_eq!(Some(5), rec(&mut env, 5));
//     }
//
//     #[test]
//     fn test_env_recursive_local_def() {
//         let mut env: Lenv<&str, i32> = Lenv::new();
//         //env.put("final".to_string(), 5);
//         assert_eq!(Some(11), rec_local_def(&mut env, 5));
//     }
//
// }
