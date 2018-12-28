use std::collections::HashMap;
use std::hash::Hash;
use std::ptr;

#[derive(Debug, PartialEq)]
pub struct Env<T: Eq + Hash, U: Clone> {
    parent: *mut Env<T, U>,
    data: Box<HashMap<T, U>>,
}

impl<T: Eq + Hash, U: Clone> Env<T, U> {
    pub fn new() -> Env<T, U> {
        Env {
            parent: ptr::null_mut(),
            data: Box::new(HashMap::new()),
        }
    }

    pub fn put(&mut self, key: T, val: U) {
        self.data.insert(key, val);
    }

    pub fn def(&mut self, key: T, val: U) {
        if !self.parent.is_null() {
            unsafe { (*self.parent).def(key, val) }
        } else {
            self.put(key, val);
        }
    }

    pub fn get(&self, key: T) -> Option<U> {
        if let Some(val) = self.data.get(&key) {
            Some(val.clone())
        } else {
            if !self.parent.is_null() {
                unsafe { (*self.parent).get(key) }
            } else {
                None
            }
        }
    }

    pub fn set_parent(&mut self, parent: &mut Env<T, U>) {
        self.parent = parent as *mut Env<T, U>;
    }
}

impl<T: Eq + Hash, U: Clone> Drop for Env<T, U> {
    fn drop(&mut self) {
        self.parent = ptr::null_mut()
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    fn rec(env: &mut Env<&str, i32>, mut val: i32) -> Option<i32> {
        if val == 0 {
            env.get("final")
        } else {
            let mut local_env: Env<&str, i32> = Env::new();
            local_env.set_parent(env);
            val -= 1;
            rec(&mut local_env, val)
        }
    }

    fn rec_local_def(env: &mut Env<&str, i32>, mut val: i32) -> Option<i32> {
        if val == 0 {
            env.get("local_final")
        } else {
            let mut local_env: Env<&str, i32> = Env::new();
            local_env.set_parent(env);
            local_env.def("local_final", val + 10);
            val -= 1;
            rec_local_def(&mut local_env, val)
        }
    }

    #[test]
    fn test_env() {
        let mut env: Env<&str, i32> = Env::new();
        let mut child: Env<&str, i32> = Env::new();
        child.set_parent(&mut env);
        env.put("Key", 1);
        env.def("Jey", 2);
        child.def("Xey", 3);

        assert_eq!(Some(1), child.get("Key"));
        assert_eq!(Some(2), child.get("Jey"));
        assert_eq!(Some(3), child.get("Xey"));
    }

    #[test]
    fn test_env_recursive() {
        let mut env: Env<&str, i32> = Env::new();
        env.put("final", 5);
        assert_eq!(Some(5), rec(&mut env, 5));
    }

    #[test]
    fn test_env_recursive_local_def() {
        let mut env: Env<&str, i32> = Env::new();
        //env.put("final".to_string(), 5);
        assert_eq!(Some(11), rec_local_def(&mut env, 5));
    }

}
