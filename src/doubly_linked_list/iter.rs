#[cfg(test)]
mod unit_tests;

use crate::NodeLink;
use std::marker::PhantomData;

#[derive(Debug)]
pub struct Iter<'a, T> {
    pub(super) curr: Option<NodeLink<T>>,
    // TODO: How to make volatile?
    #[allow(unused)]
    pub(super) rc: Option<NodeLink<T>>,
    pub(super) phantom: PhantomData<&'a T>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.curr.take().and_then(|el_ref| {
            self.curr = el_ref.borrow().next.clone();
            unsafe { ((&**el_ref.borrow()) as *const T).as_ref() }
        })
    }
}

impl<'a, T: PartialEq> PartialEq for Iter<'a, T> {
    fn eq(&self, rhs: &Self) -> bool {
        self.curr == rhs.curr &&
            self.rc == rhs.rc
    }
}
