use std::{
    cell::RefCell,
    ops::Deref,
    rc::Rc,
};
use crate::{
    Node,
    WeakLink,
};
#[derive(Debug)]
pub struct NodeLink<T>(pub(crate) Rc<RefCell<Node<T>>>);

impl<T> NodeLink<T> {
    #[inline]
    pub(crate) fn new(node: Node<T>) -> Self {
        Self(Rc::new(RefCell::new(node)))
    }

    #[inline]
    pub(crate) fn from_strong(link: Rc<RefCell<Node<T>>>) -> Self {
        Self(link)
    }

    #[inline]
    pub(crate) fn to_weak(&self) -> WeakLink<T> {
        WeakLink::from_weak(Rc::downgrade(&self.0))
    }
}

impl<T> Clone for NodeLink<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T> Deref for NodeLink<T> {
    type Target = Rc<RefCell<Node<T>>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: PartialEq> PartialEq for NodeLink<T> {
    fn eq(&self, rhs: &Self) -> bool {
        self == rhs
    }
}

