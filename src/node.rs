#[cfg(test)]
mod unit_tests;
use std::{
    cell::{
        RefCell,
    },
    cmp::Ordering,
    ops::Deref,
    rc::{
        Rc,
        Weak,
    },
};

#[derive(Debug)]
pub struct Node<T> {
    pub(super) next: Option<NodeLink<T>>,
    pub(super) prev: Option<WeakLink<T>>,
    data: T,
}

impl<T> Node<T> {
    pub fn new(data: T) -> Self {
        Self {
            next: None,
            prev: None,
            data,
        }
    }
}

impl<T> Deref for Node<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T: PartialEq> PartialEq for Node<T> {
    fn eq(&self, rhs: &Self) -> bool {
        self.data == rhs.data
    }
}

impl<T: PartialOrd> PartialOrd for Node<T> {
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        self.data.partial_cmp(&rhs.data)
    }
}

#[derive(Debug)]
pub struct NodeLink<T>(Rc<RefCell<Node<T>>>);

impl<T> NodeLink<T> {
    #[inline]
    pub(super) fn new(node: Node<T>) -> Self {
        Self(Rc::new(RefCell::new(node)))
    }

    #[inline]
    pub(super) fn to_weak(&self) -> WeakLink<T> {
        WeakLink(Rc::downgrade(&self.0))
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

#[derive(Debug)]
pub struct WeakLink<T>(Weak<RefCell<Node<T>>>);

impl<T> Clone for WeakLink<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T> Deref for WeakLink<T> {
    type Target = Weak<RefCell<Node<T>>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: PartialEq> PartialEq for WeakLink<T> {
    fn eq(&self, rhs: &Self) -> bool {
        self == rhs
    }
}
