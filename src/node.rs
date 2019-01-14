#[cfg(test)]
mod unit_tests;
use std::{
    cell::{
        RefCell,
    },
    cmp::Ordering,
    marker::PhantomData,
    ops::Deref,
    rc::{
        Rc,
        Weak,
    },
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

#[derive(Debug)]
pub struct NodeLink<'a, T>(pub(super) Rc<RefCell<Node<'a, T>>>);

impl<'a, T> NodeLink<'a, T> {
    #[inline]
    pub(super) fn new(node: Node<'a, T>) -> Self {
        Self(Rc::new(RefCell::new(node)))
    }

    #[inline]
    pub(super) fn from_strong(link: Rc<RefCell<Node<'a, T>>>) -> Self {
        Self(link)
    }

    #[inline]
    pub(super) fn to_weak(&self) -> WeakLink<'a, T> {
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

#[derive(Debug)]
pub struct WeakLink<'a, T>(Weak<RefCell<Node<'a, T>>>, PhantomData<&'a T>);

impl<'a, T> WeakLink<'a, T> {
    #[inline]
    pub(super) fn from_weak(weak_link: Weak<RefCell<Node<'a, T>>>) -> Self {
        Self(weak_link, PhantomData)
    }

    #[inline]
    pub(super) fn to_strong(&self) -> Option<NodeLink<'a, T>> {
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

