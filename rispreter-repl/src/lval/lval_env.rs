/// A custom forest tree got from github.com/SimonSapin/rust-forest

use std::cell::{self, RefCell};
use std::fmt;
use std::ops::{Deref, DerefMut};
use std::rc::{Rc, Weak};
use crate::lval::lval_def::Lval;
use crate::lval::lval_builtin::Lbuiltin;
use std::collections::HashMap;

/// A reference to a node holding a value of type `T`. Nodes form a tree.
///
/// Internally, this uses reference counting for lifetime tracking
/// and `std::cell::RefCell` for interior mutability.
///
/// **Note:** Cloning a `Lenv` only increments a reference count. It does not copy the data.
#[derive(PartialEq)]
pub struct Lenv(Rc<RefCell<Node>>);

struct Node {
    parent: WeakLink,
    first_child: Link,
    last_child: WeakLink,
    previous_sibling: WeakLink,
    next_sibling: Link,
    data: HashMap<String, Lval>,
}

type Link = Option<Rc<RefCell<Node>>>;
type WeakLink = Option<Weak<RefCell<Node>>>;


impl PartialEq for Node {
    fn eq(&self, other: &Node) -> bool {
        false
    }
}

fn same_rc<T>(a: &Rc<T>, b: &Rc<T>) -> bool {
    let a: *const T = &**a;
    let b: *const T = &**b;
    a == b
}


/// Cloning a `Lenv` only increments a reference count. It does not copy the data.
impl Clone for Lenv {
    fn clone(&self) -> Lenv {
        Lenv(self.0.clone())
    }
}

impl fmt::Debug for Lenv {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for child in self.children() {
            println!("\nchild:");
            fmt::Debug::fmt(&*child.borrow(), f)?
        }
        println!("\nparent:");
        fmt::Debug::fmt(&*self.borrow(), f)
    }
}

macro_rules! try_opt {
    ($expr: expr) => {
        match $expr {
            Some(value) => value,
            None => return None
        }
    }
}

impl Lenv {
    pub fn add_builtin(&mut self, name: &str, func: Lbuiltin) {
            let lval = Lval::lval_fun(func);
            self.borrow_mut().insert(name.to_string(), lval);
    }

    pub fn put(&self, k: String, v: Lval) {
        self.borrow_mut().insert(k, v);
    }

    pub fn def(&self, k: String, v: Lval){
        match self.parent() {
            Some(parent) => {
                parent.def(k, v)
            },
            None => {
                self.put(k, v)
            }
        }
    }

    pub fn get(&self, k: String) -> Lval {
        match self.borrow().get(&k) {
            Some(v) => {
                v.clone()
            },
            None => {
                match self.parent() {
                    Some(parent) => {
                        parent.get(k)
                    },
                    None => {
                        Lval::lval_err(format!("Can't fing {}", k))
                    }
                }
            }
        }
    }

    pub fn contains(&self, k: String) -> bool {
            self.borrow().contains_key(&k)
    }

}

impl Lenv {
    /// Create a new node from its associated data.
    pub fn new() -> Lenv {
        Lenv(Rc::new(RefCell::new(Node {
            parent: None,
            first_child: None,
            last_child: None,
            previous_sibling: None,
            next_sibling: None,
            data: HashMap::new(),
        })))
    }

    /// Return a reference to the parent node, unless this node is the root of the tree.
    ///
    /// # Panics
    ///
    /// Panics if the node is currently mutability borrowed.
    pub fn parent(&self) -> Option<Lenv> {
        Some(Lenv(try_opt!(try_opt!(self.0.borrow().parent.as_ref()).upgrade())))
    }

    /// Return a reference to the first child of this node, unless it has no child.
    ///
    /// # Panics
    ///
    /// Panics if the node is currently mutability borrowed.
    pub fn first_child(&self) -> Option<Lenv> {
        Some(Lenv(try_opt!(self.0.borrow().first_child.as_ref()).clone()))
    }

    /// Return a reference to the last child of this node, unless it has no child.
    ///
    /// # Panics
    ///
    /// Panics if the node is currently mutability borrowed.
    pub fn last_child(&self) -> Option<Lenv> {
        Some(Lenv(try_opt!(try_opt!(self.0.borrow().last_child.as_ref()).upgrade())))
    }

    /// Return a reference to the previous sibling of this node, unless it is a first child.
    ///
    /// # Panics
    ///
    /// Panics if the node is currently mutability borrowed.
    pub fn previous_sibling(&self) -> Option<Lenv> {
        Some(Lenv(try_opt!(try_opt!(self.0.borrow().previous_sibling.as_ref()).upgrade())))
    }

    /// Return a reference to the previous sibling of this node, unless it is a first child.
    ///
    /// # Panics
    ///
    /// Panics if the node is currently mutability borrowed.
    pub fn next_sibling(&self) -> Option<Lenv> {
        Some(Lenv(try_opt!(self.0.borrow().next_sibling.as_ref()).clone()))
    }

    /// Return a shared reference to this node’s data
    ///
    /// # Panics
    ///
    /// Panics if the node is currently mutability borrowed.
    pub fn borrow(&self) -> Ref{
        Ref { _ref: self.0.borrow() }
    }

    /// Return a unique/mutable reference to this node’s data
    ///
    /// # Panics
    ///
    /// Panics if the node is currently borrowed.
    pub fn borrow_mut(&self) -> RefMut {
        RefMut { _ref: self.0.borrow_mut() }
    }

    /// Returns whether two references point to the same node.
    pub fn same_node(&self, other: &Lenv) -> bool {
        same_rc(&self.0, &other.0)
    }

    /// Return an iterator of references to this node and its ancestors.
    ///
    /// Call `.next().unwrap()` once on the iterator to skip the node itself.
    pub fn ancestors(&self) -> Ancestors {
        Ancestors(Some(self.clone()))
    }

    /// Return an iterator of references to this node and the siblings before it.
    ///
    /// Call `.next().unwrap()` once on the iterator to skip the node itself.
    pub fn preceding_siblings(&self) -> PrecedingSiblings {
        PrecedingSiblings(Some(self.clone()))
    }

    /// Return an iterator of references to this node and the siblings after it.
    ///
    /// Call `.next().unwrap()` once on the iterator to skip the node itself.
    pub fn following_siblings(&self) -> FollowingSiblings {
        FollowingSiblings(Some(self.clone()))
    }

    /// Return an iterator of references to this node’s children.
    ///
    /// # Panics
    ///
    /// Panics if the node is currently mutability borrowed.
    pub fn children(&self) -> Children {
        Children(self.first_child())
    }

    /// Return an iterator of references to this node’s children, in reverse order.
    ///
    /// # Panics
    ///
    /// Panics if the node is currently mutability borrowed.
    pub fn reverse_children(&self) -> ReverseChildren {
        ReverseChildren(self.last_child())
    }

    /// Return an iterator of references to this node and its descendants, in tree order.
    ///
    /// Parent nodes appear before the descendants.
    /// Call `.next().unwrap()` once on the iterator to skip the node itself.
    pub fn descendants(&self) -> Descendants {
        Descendants(self.traverse())
    }

    /// Return an iterator of references to this node and its descendants, in tree order.
    pub fn traverse(&self) -> Traverse {
        Traverse {
            root: self.clone(),
            next: Some(NodeEdge::Start(self.clone())),
        }
    }

    /// Return an iterator of references to this node and its descendants, in tree order.
    pub fn reverse_traverse(&self) -> ReverseTraverse {
        ReverseTraverse {
            root: self.clone(),
            next: Some(NodeEdge::End(self.clone())),
        }
    }

    /// Detach a node from its parent and siblings. Children are not affected.
    ///
    /// # Panics
    ///
    /// Panics if the node or one of its adjoining nodes is currently borrowed.
    pub fn detach(&self) {
        self.0.borrow_mut().detach();
    }

    /// Append a new child to this node, after existing children.
    ///
    /// # Panics
    ///
    /// Panics if the node, the new child, or one of their adjoining nodes is currently borrowed.
    pub fn append(&self, new_child: Lenv) {
        let mut self_borrow = self.0.borrow_mut();
        let mut last_child_opt = None;
        {
            let mut new_child_borrow = new_child.0.borrow_mut();
            new_child_borrow.detach();
            new_child_borrow.parent = Some(Rc::downgrade(&self.0));
            if let Some(last_child_weak) = self_borrow.last_child.take() {
                if let Some(last_child_strong) = last_child_weak.upgrade() {
                    new_child_borrow.previous_sibling = Some(last_child_weak);
                    last_child_opt = Some(last_child_strong);
                }
            }
            self_borrow.last_child = Some(Rc::downgrade(&new_child.0));
        }

        if let Some(last_child_strong) = last_child_opt {
            let mut last_child_borrow = last_child_strong.borrow_mut();
            debug_assert!(last_child_borrow.next_sibling.is_none());
            last_child_borrow.next_sibling = Some(new_child.0);
        } else {
            // No last child
            debug_assert!(self_borrow.first_child.is_none());
            self_borrow.first_child = Some(new_child.0);
        }
    }

    /// Prepend a new child to this node, before existing children.
    ///
    /// # Panics
    ///
    /// Panics if the node, the new child, or one of their adjoining nodes is currently borrowed.
    pub fn prepend(&self, new_child: Lenv) {
        let mut self_borrow = self.0.borrow_mut();
        {
            let mut new_child_borrow = new_child.0.borrow_mut();
            new_child_borrow.detach();
            new_child_borrow.parent = Some(Rc::downgrade(&self.0));
            match self_borrow.first_child.take() {
                Some(first_child_strong) => {
                    {
                        let mut first_child_borrow = first_child_strong.borrow_mut();
                        debug_assert!(first_child_borrow.previous_sibling.is_none());
                        first_child_borrow.previous_sibling = Some(Rc::downgrade(&new_child.0));
                    }
                    new_child_borrow.next_sibling = Some(first_child_strong);
                }
                None => {
                    debug_assert!(self_borrow.first_child.is_none());
                    self_borrow.last_child = Some(Rc::downgrade(&new_child.0));
                }
            }
        }
        self_borrow.first_child = Some(new_child.0);
    }

    /// Insert a new sibling after this node.
    ///
    /// # Panics
    ///
    /// Panics if the node, the new sibling, or one of their adjoining nodes is currently borrowed.
    pub fn insert_after(&self, new_sibling: Lenv) {
        let mut self_borrow = self.0.borrow_mut();
        {
            let mut new_sibling_borrow = new_sibling.0.borrow_mut();
            new_sibling_borrow.detach();
            new_sibling_borrow.parent = self_borrow.parent.clone();
            new_sibling_borrow.previous_sibling = Some(Rc::downgrade(&self.0));
            match self_borrow.next_sibling.take() {
                Some(next_sibling_strong) => {
                    {
                        let mut next_sibling_borrow = next_sibling_strong.borrow_mut();
                        debug_assert!({
                            let weak = next_sibling_borrow.previous_sibling.as_ref().unwrap();
                            same_rc(&weak.upgrade().unwrap(), &self.0)
                        });
                        next_sibling_borrow.previous_sibling = Some(Rc::downgrade(&new_sibling.0));
                    }
                    new_sibling_borrow.next_sibling = Some(next_sibling_strong);
                }
                None => {
                    if let Some(parent_ref) = self_borrow.parent.as_ref() {
                        if let Some(parent_strong) = parent_ref.upgrade() {
                            let mut parent_borrow = parent_strong.borrow_mut();
                            parent_borrow.last_child = Some(Rc::downgrade(&new_sibling.0));
                        }
                    }
                }
            }
        }
        self_borrow.next_sibling = Some(new_sibling.0);
    }

    /// Insert a new sibling before this node.
    ///
    /// # Panics
    ///
    /// Panics if the node, the new sibling, or one of their adjoining nodes is currently borrowed.
    pub fn insert_before(&self, new_sibling: Lenv) {
        let mut self_borrow = self.0.borrow_mut();
        let mut previous_sibling_opt = None;
        {
            let mut new_sibling_borrow = new_sibling.0.borrow_mut();
            new_sibling_borrow.detach();
            new_sibling_borrow.parent = self_borrow.parent.clone();
            new_sibling_borrow.next_sibling = Some(self.0.clone());
            if let Some(previous_sibling_weak) = self_borrow.previous_sibling.take() {
                if let Some(previous_sibling_strong) = previous_sibling_weak.upgrade() {
                    new_sibling_borrow.previous_sibling = Some(previous_sibling_weak);
                    previous_sibling_opt = Some(previous_sibling_strong);
                }
            }
            self_borrow.previous_sibling = Some(Rc::downgrade(&new_sibling.0));
        }

        if let Some(previous_sibling_strong) = previous_sibling_opt {
            let mut previous_sibling_borrow = previous_sibling_strong.borrow_mut();
            debug_assert!({
                let rc = previous_sibling_borrow.next_sibling.as_ref().unwrap();
                same_rc(rc, &self.0)
            });
            previous_sibling_borrow.next_sibling = Some(new_sibling.0);
        } else {
            // No previous sibling.
            if let Some(parent_ref) = self_borrow.parent.as_ref() {
                if let Some(parent_strong) = parent_ref.upgrade() {
                    let mut parent_borrow = parent_strong.borrow_mut();
                    parent_borrow.first_child = Some(new_sibling.0);
                }
            }
        }
    }
}

/// Wraps a `std::cell::Ref` for a node’s data.
pub struct Ref<'a> {
    _ref: cell::Ref<'a, Node>
}

/// Wraps a `std::cell::RefMut` for a node’s data.
pub struct RefMut<'a> {
    _ref: cell::RefMut<'a, Node>
}

impl<'a> Deref for Ref<'a> {
    type Target = HashMap<String, Lval>;
    fn deref(&self) -> & HashMap<String, Lval> { &self._ref.data }
}

impl<'a> Deref for RefMut<'a> {
    type Target = HashMap<String, Lval>;
    fn deref(&self) -> &HashMap<String, Lval> { &self._ref.data }
}

impl<'a> DerefMut for RefMut<'a> {
    fn deref_mut(&mut self) -> &mut HashMap<String, Lval> { &mut self._ref.data }
}


impl Node {
    /// Detach a node from its parent and siblings. Children are not affected.
    fn detach(&mut self) {
        let parent_weak = self.parent.take();
        let previous_sibling_weak = self.previous_sibling.take();
        let next_sibling_strong = self.next_sibling.take();

        let previous_sibling_opt = previous_sibling_weak.as_ref().and_then(|weak| weak.upgrade());

        if let Some(next_sibling_ref) = next_sibling_strong.as_ref() {
            let mut next_sibling_borrow = next_sibling_ref.borrow_mut();
            next_sibling_borrow.previous_sibling = previous_sibling_weak;
        } else if let Some(parent_ref) = parent_weak.as_ref() {
            if let Some(parent_strong) = parent_ref.upgrade() {
                let mut parent_borrow = parent_strong.borrow_mut();
                parent_borrow.last_child = previous_sibling_weak;
            }
        }

        if let Some(previous_sibling_strong) = previous_sibling_opt {
            let mut previous_sibling_borrow = previous_sibling_strong.borrow_mut();
            previous_sibling_borrow.next_sibling = next_sibling_strong;
        } else if let Some(parent_ref) = parent_weak.as_ref() {
            if let Some(parent_strong) = parent_ref.upgrade() {
                let mut parent_borrow = parent_strong.borrow_mut();
                parent_borrow.first_child = next_sibling_strong;
            }
        }
    }
}


macro_rules! impl_node_iterator {
    ($name: ident, $next: expr) => {
        impl Iterator for $name {
            type Item = Lenv;

            /// # Panics
            ///
            /// Panics if the node about to be yielded is currently mutability borrowed.
            fn next(&mut self) -> Option<Lenv> {
                match self.0.take() {
                    Some(node) => {
                        self.0 = $next(&node);
                        Some(node)
                    }
                    None => None
                }
            }
        }
    }
}

/// An iterator of references to the ancestors a given node.
pub struct Ancestors(Option<Lenv>);
impl_node_iterator!(Ancestors, |node: &Lenv| node.parent());

/// An iterator of references to the siblings before a given node.
pub struct PrecedingSiblings(Option<Lenv>);
impl_node_iterator!(PrecedingSiblings, |node: &Lenv| node.previous_sibling());

/// An iterator of references to the siblings after a given node.
pub struct FollowingSiblings(Option<Lenv>);
impl_node_iterator!(FollowingSiblings, |node: &Lenv| node.next_sibling());

/// An iterator of references to the children of a given node.
pub struct Children(Option<Lenv>);
impl_node_iterator!(Children, |node: &Lenv| node.next_sibling());

/// An iterator of references to the children of a given node, in reverse order.
pub struct ReverseChildren(Option<Lenv>);
impl_node_iterator!(ReverseChildren, |node: &Lenv| node.previous_sibling());


/// An iterator of references to a given node and its descendants, in tree order.
pub struct Descendants(Traverse);

impl Iterator for Descendants {
    type Item = Lenv;

    /// # Panics
    ///
    /// Panics if the node about to be yielded is currently mutability borrowed.
    fn next(&mut self) -> Option<Lenv> {
        loop {
            match self.0.next() {
                Some(NodeEdge::Start(node)) => return Some(node),
                Some(NodeEdge::End(_)) => {}
                None => return None
            }
        }
    }
}


#[derive(Debug, Clone)]
pub enum NodeEdge {
    /// Indicates that start of a node that has children.
    /// Yielded by `Traverse::next` before the node’s descendants.
    /// In HTML or XML, this corresponds to an opening tag like `<div>`
    Start(Lenv),

    /// Indicates that end of a node that has children.
    /// Yielded by `Traverse::next` after the node’s descendants.
    /// In HTML or XML, this corresponds to a closing tag like `</div>`
    End(Lenv),
}


/// An iterator of references to a given node and its descendants, in tree order.
pub struct Traverse {
    root: Lenv,
    next: Option<NodeEdge>,
}

impl Iterator for Traverse {
    type Item = NodeEdge;

    /// # Panics
    ///
    /// Panics if the node about to be yielded is currently mutability borrowed.
    fn next(&mut self) -> Option<NodeEdge> {
        match self.next.take() {
            Some(item) => {
                self.next = match item {
                    NodeEdge::Start(ref node) => {
                        match node.first_child() {
                            Some(first_child) => Some(NodeEdge::Start(first_child)),
                            None => Some(NodeEdge::End(node.clone()))
                        }
                    }
                    NodeEdge::End(ref node) => {
                        if node.same_node(&self.root) {
                            None
                        } else {
                            match node.next_sibling() {
                                Some(next_sibling) => Some(NodeEdge::Start(next_sibling)),
                                None => match node.parent() {
                                    Some(parent) => Some(NodeEdge::End(parent)),

                                    // `node.parent()` here can only be `None`
                                    // if the tree has been modified during iteration,
                                    // but silently stoping iteration
                                    // seems a more sensible behavior than panicking.
                                    None => None
                                }
                            }
                        }
                    }
                };
                Some(item)
            }
            None => None
        }
    }
}

/// An iterator of references to a given node and its descendants, in reverse tree order.
pub struct ReverseTraverse {
    root: Lenv,
    next: Option<NodeEdge>,
}

impl Iterator for ReverseTraverse {
    type Item = NodeEdge;

    /// # Panics
    ///
    /// Panics if the node about to be yielded is currently mutability borrowed.
    fn next(&mut self) -> Option<NodeEdge> {
        match self.next.take() {
            Some(item) => {
                self.next = match item {
                    NodeEdge::End(ref node) => {
                        match node.last_child() {
                            Some(last_child) => Some(NodeEdge::End(last_child)),
                            None => Some(NodeEdge::Start(node.clone()))
                        }
                    }
                    NodeEdge::Start(ref node) => {
                        if node.same_node(&self.root) {
                            None
                        } else {
                            match node.previous_sibling() {
                                Some(previous_sibling) => Some(NodeEdge::End(previous_sibling)),
                                None => match node.parent() {
                                    Some(parent) => Some(NodeEdge::Start(parent)),

                                    // `node.parent()` here can only be `None`
                                    // if the tree has been modified during iteration,
                                    // but silently stoping iteration
                                    // seems a more sensible behavior than panicking.
                                    None => None
                                }
                            }
                        }
                    }
                };
                Some(item)
            }
            None => None
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     #[test]
//     fn it_works() {
//         struct DropTracker<'a>(&'a cell::Cell<u32>);
//         impl<'a> Drop for DropTracker<'a> {
//             fn drop(&mut self) {
//                 self.0.set(self.0.get() + 1);
//             }
//         }
//
//         let mut new_counter = 0;
//         let drop_counter = cell::Cell::new(0);
//         let mut new = || {
//             new_counter += 1;
//             Lenv::new((new_counter, DropTracker(&drop_counter)))
//         };
//
//         {
//             let a = new();  // 1
//             a.append(new());  // 2
//             a.append(new());  // 3
//             a.prepend(new());  // 4
//             let b = new();  // 5
//             b.append(a.clone());
//             a.insert_before(new());  // 6
//             a.insert_before(new());  // 7
//             a.insert_after(new());  // 8
//             a.insert_after(new());  // 9
//             let c = new();  // 10
//             b.append(c.clone());
//
//             assert_eq!(drop_counter.get(), 0);
//             c.previous_sibling().unwrap().detach();
//             assert_eq!(drop_counter.get(), 1);
//
//             assert_eq!(b.descendants().map(|node| {
//                 let borrow = node.borrow();
//                 borrow.0
//             }).collect::<Vec<_>>(), [
//                 5, 6, 7, 1, 4, 2, 3, 9, 10
//             ]);
//         }
//
//         assert_eq!(drop_counter.get(), 10);
//     }
// }
