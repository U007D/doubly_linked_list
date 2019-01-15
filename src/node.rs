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
pub struct Node<'a, T> {
    pub(super) next: Option<NodeLink<'a, T>>,
    pub(super) prev: Option<WeakLink<'a, T>>,
    pub(super) data: T,
}

impl<'a, T> Node<'a, T> {
    pub fn new(data: T) -> Self {
        Self {
            next: None,
            prev: None,
            data,
        }
    }
}

impl<'a, T> Deref for Node<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<'a, T: PartialEq> PartialEq for Node<'a, T> {
    fn eq(&self, rhs: &Self) -> bool {
        self.data == rhs.data
    }
}

impl<'a, T: PartialOrd> PartialOrd for Node<'a, T> {
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        self.data.partial_cmp(&rhs.data)
    }
}
