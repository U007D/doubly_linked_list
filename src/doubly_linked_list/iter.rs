#[cfg(test)]
mod unit_tests;
use crate::NodeLink;

#[derive(Debug)]
pub struct Iter<T>(pub(super) Option<NodeLink<T>>);

impl<T> Iterator for Iter<T> {
    type Item = NodeLink<T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.take()
              .and_then(|link| {
                  self.0 = link.borrow().next.clone();
                  Some(link)
              })
    }
}

impl<T: PartialEq> PartialEq for Iter<T> {
    fn eq(&self, rhs: &Self) -> bool {
        self.0 == rhs.0
    }
}
