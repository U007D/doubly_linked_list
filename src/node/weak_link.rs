use std::{
    cell::RefCell,
    marker::PhantomData,
    rc::Weak,
};
use crate::{
    Node,
    NodeLink,
};

#[derive(Debug)]
pub struct WeakLink<'a, T>(Weak<RefCell<Node<'a, T>>>, PhantomData<&'a T>);

impl<'a, T> WeakLink<'a, T> {
    #[inline]
    pub(crate) fn from_weak(weak_link: Weak<RefCell<Node<'a, T>>>) -> Self {
        Self(weak_link, PhantomData)
    }

    #[inline]
    pub(crate) fn to_strong(&self) -> Option<NodeLink<'a, T>> {
        Weak::upgrade(&self.0).and_then(|link| Some(NodeLink::from_strong(link)))
    }
}

impl<'a, T> Clone for WeakLink<'a, T> {
    fn clone(&self) -> Self {
        Self::from_weak(self.0.clone())
    }
}

impl<'a, T: PartialEq> PartialEq for WeakLink<'a, T> {
    fn eq(&self, rhs: &Self) -> bool {
        self == rhs
    }
}

