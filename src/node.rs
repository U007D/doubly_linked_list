mod node_link;
mod weak_link;
#[cfg(test)]
mod unit_tests;
pub use self::{
    node_link::NodeLink,
    weak_link::WeakLink,
};
use std::{
    cmp::Ordering,
    ops::Deref,
};

#[derive(Debug)]
pub struct Node<T> {
    pub(super) next: Option<NodeLink<T>>,
    pub(super) prev: Option<WeakLink<T>>,
    pub(super) data: T,
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

impl<T> PartialEq for Node<T> {
    fn eq(&self, rhs: &Self) -> bool {
        self as *const Node<T> == rhs as *const Node<T>
    }
}

impl<T: PartialOrd> PartialOrd for Node<T> {
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        self.data.partial_cmp(&rhs.data)
    }
}
