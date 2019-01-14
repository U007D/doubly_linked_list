mod iter;
#[cfg(test)]
mod unit_tests;
use crate::{
    Error,
    Node,
    Result,
    StrongLink,
    WeakLink,
};
pub use self::iter::Iter;
use std::{
    marker::PhantomData,
    rc::Rc,
};

/// The current implementation of `DoublyLinkedList` is not thread-safe.  Specifically, `.next` and `.prev` link
/// manipulations are not synchronized, nor is access to node data provided by `Iter::next`.
#[derive(Debug)]
pub struct DoublyLinkedList<'a, T> {
    head: Option<StrongLink<'a, T>>,
    tail: Option<WeakLink<'a, T>>,
}

impl<'a, T> DoublyLinkedList<'a, T> {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn iter(&self) -> Iter<'a, T> {
        Iter {
            curr: self.head.clone(),
            // Since next returns `&T` scoped to `Iter<'a>`, it would be possible for a `Node<'a, T>` to be dropped while
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

    pub fn push_back(&mut self, data: T) -> &mut Self {
        let mut node = Node::new(data);
        let old_tail = self.tail.take();
        node.prev = old_tail.clone();
        let node_link = StrongLink::new(node);
        self.tail = Some(node_link.to_weak());
        match old_tail {
            Some(prev) => prev.to_strong()
                // `.expect()` cannot fail in this circumstance because the node being referenced has
                // a live (strong) reference pointing to it (either the `Node<'a, T>::next` previous to it
                // or the `List::head` field, if that node is first in the list).
                              .expect("Internal error: DoublyLinkedList impl is deleting .next before .prev")
                              .borrow_mut()
                              .next = Some(node_link),
            None => self.head = Some(node_link),
        };
        self
    }

    pub fn pop_front(&mut self) -> Result<T> {
        self.head
            .take()
            .ok_or(Error::EmptyList)
            .and_then(|link| {
                self.head = link.borrow()
                                .next
                                .clone()
                                .or_else(|| {
                                    self.tail = None;
                                    None
                                });
                Rc::try_unwrap(link.0).and_then(|ref_cell| Ok(ref_cell.into_inner().data))
                                      .or_else(|rc| Err(Error::ExistingLiveReferences(Rc::strong_count(&rc))))
            })
    }

    pub fn push_front(&mut self, data: T) -> &mut Self {
        let mut node = Node::new(data);
        let old_head = self.head.take();
        node.next = old_head.clone();
        let node_link = StrongLink::new(node);
        self.head = Some(node_link.clone());
        match old_head {
            Some(head) => head.borrow_mut()
                              .prev = Some(node_link.to_weak()),
            None => self.tail = Some(node_link.to_weak()),
        };
        self
    }
}

impl<'a, T> Default for DoublyLinkedList<'a, T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, T: PartialEq> PartialEq for DoublyLinkedList<'a, T> {
    fn eq(&self, rhs: &Self) -> bool {
        self.head == rhs.head &&
        self.tail == rhs.tail
    }
}
