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
pub struct NodeLink<'a, T>(pub(crate) Rc<RefCell<Node<'a, T>>>);

impl<'a, T> NodeLink<'a, T> {
    #[inline]
    pub(crate) fn new(node: Node<'a, T>) -> Self {
        Self(Rc::new(RefCell::new(node)))
    }

    #[inline]
    pub(crate) fn from_strong(link: Rc<RefCell<Node<'a, T>>>) -> Self {
        Self(link)
    }

    #[inline]
    pub(crate) fn to_weak(&self) -> WeakLink<'a, T> {
        WeakLink::from_weak(Rc::downgrade(&self.0))
    }
}

impl<'a, T> Clone for NodeLink<'a, T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<'a, T> Deref for NodeLink<'a, T> {
    type Target = Rc<RefCell<Node<'a, T>>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a, T: PartialEq> PartialEq for NodeLink<'a, T> {
    fn eq(&self, rhs: &Self) -> bool {
        self == rhs
    }
}

