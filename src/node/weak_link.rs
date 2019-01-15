use std::{
    cell::RefCell,
    rc::Weak,
};
use crate::{
    Node,
    NodeLink,
};

#[derive(Debug)]
pub struct WeakLink<T>(Weak<RefCell<Node<T>>>);

impl<T> WeakLink<T> {
    #[inline]
    pub(crate) fn from_weak(weak_link: Weak<RefCell<Node<T>>>) -> Self {
        Self(weak_link)
    }

    #[inline]
    pub(crate) fn to_strong(&self) -> Option<NodeLink<T>> {
        Weak::upgrade(&self.0).and_then(|link| Some(NodeLink::from_strong(link)))
    }
}

impl<T> Clone for WeakLink<T> {
    fn clone(&self) -> Self {
        Self::from_weak(self.0.clone())
    }
}

impl<T: PartialEq> PartialEq for WeakLink<T> {
    fn eq(&self, rhs: &Self) -> bool {
        self == rhs
    }
}

