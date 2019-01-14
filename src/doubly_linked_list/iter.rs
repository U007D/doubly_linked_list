#[cfg(test)]
mod unit_tests;
use crate::{
    Node,
    NodeLink,
};
use std::{
    cell::RefCell,
    rc::Rc,
};

#[derive(Debug)]
pub struct Iter<'a, T>(pub(super) Option<NodeLink<'a, T>>);

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = Rc<RefCell<Node<'a, T>>>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.take()
              .and_then(|link| {
                  self.0 = link.borrow().next.clone();
                  Some(link.0)
              })
    }
}

impl<'a, T: PartialEq> PartialEq for Iter<'a, T> {
    fn eq(&self, rhs: &Self) -> bool {
        self.0 == rhs.0
    }
}
