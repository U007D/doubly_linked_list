mod iter;
#[cfg(test)]
mod unit_tests;
use crate::{
    Node,
    NodeLink,
    WeakLink,
};
pub use self::iter::Iter;
use std::marker::PhantomData;

/// The current implementation of `DoublyLinkedList` is not thread-safe.  Specifically, `.next` and `.prev` link
/// manipulations are not synchronized, nor is access to node data provided by `Iter::next`.
#[derive(Debug)]
pub struct DoublyLinkedList<T> {
    head: Option<NodeLink<T>>,
    tail: Option<WeakLink<T>>,
}

impl<T> DoublyLinkedList<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn iter(&self) -> Iter<T> {
        Iter {
            curr: self.head.clone(),
            // Since next returns `&T` scoped to `Iter<'a>`, it would be possible for a `Node<T>` to be dropped while
            // the caller is still holding the `&T`.  Cloning and holding the `Rc` of the iterator's head node
            // ensures this cannot happen.
            rc: self.head.clone(),
            phantom: PhantomData,
        }
    }

    pub fn len(&self) -> usize {
        let mut count = 0;
        let mut link_opt = self.head.clone();
        while let Some(link) = link_opt {
            link_opt = link.borrow().next.clone();
            count += 1;
        }
        count
    }

    pub fn push_back(&mut self, mut node: Node<T>) -> &mut Self {
        let old_tail = self.tail.take();
        node.prev = old_tail.clone();
        let node_link = NodeLink::new(node);
        self.tail = Some(node_link.to_weak());
        match old_tail {
            Some(prev) => prev.upgrade()
                              // `.expect()` cannot fail in this circumstance because the node being referenced has
                              // a live (strong) reference pointing to it (either the `Node<T>::next` previous to it
                              // or the `List::head` field, if that node is first in the list).
                              .expect("Internal error: DoublyLinkedList impl is deleting .next before .prev")
                              .borrow_mut()
                              .next = Some(node_link),
            None => self.head = Some(node_link),
        };
        self
    }

    pub fn push_front(&mut self, mut node: Node<T>) -> &mut Self {
        let old_head = self.head.take();
        node.next = old_head.clone();
        let node_link = NodeLink::new(node);
        self.head = Some(node_link.clone());
        match old_head {
            Some(head) => head.borrow_mut()
                              .prev = Some(node_link.to_weak()),
            None => self.tail = Some(node_link.to_weak()),
        };
        self
    }
}

impl<T> Default for DoublyLinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: PartialEq> PartialEq for DoublyLinkedList<T> {
    fn eq(&self, rhs: &Self) -> bool {
        self.head == rhs.head &&
        self.tail == rhs.tail
    }
}
