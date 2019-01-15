mod iter;
#[cfg(test)]
mod unit_tests;
use crate::{
    consts::*,
    Error,
    Node,
    Result,
    NodeLink,
    WeakLink,
};
pub use self::iter::Iter;
use std::rc::Rc;

/// `DoublyLinkedList` represents a series of `Node`s, provides appropriate data insertion and removal methods, and
/// an permits iterating over the collection.
#[derive(Debug)]
pub struct DoublyLinkedList<T> {
    head: Option<NodeLink<T>>,
    tail: Option<WeakLink<T>>,
}

impl<T> DoublyLinkedList<T> {
    /// Constructor.
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
        }
    }

    /// Predicate revealing whether the list is empty (contains no `Node`s) or not.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Inserts `data` as a `Node` into the list positionally after the `Node` referenced by `curr`.  If `curr`
    /// represents the tail of the list, this method delegates to `push_back()`, instead, so that the
    /// `DoublyLinkedList`'s `tail` field is properly maintained.
    pub fn insert_after(&mut self, curr: NodeLink<T>, data: T) -> &mut Self {
        let old_next_opt = curr.borrow_mut().next.take();
        match old_next_opt {
            None => self.push_back(data),
            Some(old_next) => {
                let new_next = NodeLink::new(Node::new(data));

                // update new next node's previous ref
                new_next.borrow_mut().prev = old_next.borrow().prev.clone();

                // update old next node's prev ref
                old_next.borrow_mut().prev = Some(new_next.to_weak());

                // update new next node's next ref
                new_next.borrow_mut().next = Some(old_next);

                // update current node's next ref
                curr.borrow_mut().next = Some(new_next);

                self
            }
        }
    }

    /// Inserts `data` as a `Node` into the list positionally before the `Node` referenced by `curr`.  If `curr`
    /// represents the head of the list, this method delegates to `push_front()`, instead, so that the
    /// `DoublyLinkedList`'s `head` field is properly maintained.
    pub fn insert_before(&mut self, curr: NodeLink<T>, data: T) -> &mut Self {
        let old_prev_opt = curr.borrow_mut().prev.take();
        match old_prev_opt {
            None => self.push_front(data),
            Some(weak) => {
                let old_prev = weak.to_strong().expect(msg::ERR_INTERNAL_WEAK_UPGRADE_RACE);
                let new_prev = NodeLink::new(Node::new(data));

                // update current node's previous ref
                curr.borrow_mut().prev = Some(new_prev.to_weak());

                // update new previous node's previous and next refs
                new_prev.borrow_mut().prev = Some(old_prev.to_weak());
                new_prev.borrow_mut().next = old_prev.borrow().next.clone();

                // update old previous node's next ref
                old_prev.borrow_mut().next = Some(new_prev);

                self
            }
        }
    }

    /// Creates an `Iterator` permitting iteration over the collection.
    pub fn iter(&self) -> Iter<T> {
        Iter(self.head.clone())
    }

    /// Returns the number of `Node`s currently in the list.  Note: this method does not check for the case where the
    /// number of `Nodes` > usize::MAX.  In such a case, this method will panic in debug and silently wrap in release
    /// (Rust/C++/C default behavior).
    pub fn len(&self) -> usize {
        let mut count = 0;
        let mut link_opt = self.head.clone();
        while let Some(link) = link_opt {
            link_opt = link.borrow().next.clone();
            count += 1;
        }
        count
    }

    /// Removes the `Node` at the tail of the list and returns the `data` contained within.
    /// Note: Because of the way `RefCell` works, this will return an error if there is another live reference (e.g.
    /// caller also called `.iter()` and is holding a live `.borrow()`) to this and/or either of its adjacent (i.e.
    /// previous or next) `Node`s when calling this method.
    pub fn pop_back(&mut self) -> Result<T> {
        self.tail
            .take()
            .ok_or(Error::EmptyList)
            .and_then(|weak| {
                let old_tail = weak.to_strong()
                                   .expect(msg::ERR_INTERNAL_WEAK_UPGRADE_RACE);
                // Set tail to point to extracted `Node`'s predecessor
                self.tail = old_tail.borrow()
                                    .prev
                                    .clone()
                                    // If there was no predecessor, this was the only Node in the list.  After
                                    // extraction, the list will be empty
                                    .or_else(|| {
                                        self.head = None;
                                        None
                                    })
                                    // Otherwise, a predecessor exists.  Set it to be the new end-of-list `Node`
                                    .and_then(|weak| {
                                        let new_tail = weak.to_strong()
                                                           .expect(msg::ERR_INTERNAL_WEAK_UPGRADE_RACE);
                                        new_tail.borrow_mut().next = None;
                                        Some(new_tail.to_weak())
                                    });
                // Extract data from extracted `Node`.  If the extracted `Node` has outstanding live references, the
                // runtime `borrowck` will (correctly) prevent extraction and this method will return an error
                Rc::try_unwrap(old_tail.0).and_then(|ref_cell| Ok(ref_cell.into_inner().data))
                                          .or_else(|rc| Err(Error::ExistingLiveReferences(Rc::strong_count(&rc))))
            })
    }

    /// Removes the `Node` at the head of the list and returns the `data` contained within.
    /// Note: Because of the way `RefCell` works, this will return an error if there is another live reference (e.g.
    /// caller also called `.iter()` and is holding a live `.borrow()`) to this and/or either of its adjacent (i.e.
    /// previous or next) `Node`s when calling this method.
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

    /// Appends a `Node` to the end of the list.
    pub fn push_back(&mut self, data: T) -> &mut Self {
        let mut node = Node::new(data);
        let old_tail = self.tail.take();
        node.prev = old_tail.clone();
        let node_link = NodeLink::new(node);
        self.tail = Some(node_link.to_weak());
        match old_tail {
            Some(prev) => prev.to_strong()
                              .expect(msg::ERR_INTERNAL_WEAK_UPGRADE_RACE)
                              .borrow_mut()
                              .next = Some(node_link),
            None => self.head = Some(node_link),
        };
        self
    }

    /// Prepends a `Node` to the front of the list.
    pub fn push_front(&mut self, data: T) -> &mut Self {
        let mut node = Node::new(data);
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

/// Idiomatic `Default` impl for types with parameterless constructors.
impl<T> Default for DoublyLinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

/// Impl of total equality (marker trait) for `DoublyLinkedList`
impl<T: Eq> Eq for DoublyLinkedList<T> {}

/// Implementation of partial equality for `DoublyLinkedList`
impl<T: PartialEq> PartialEq for DoublyLinkedList<T> {
    fn eq(&self, rhs: &Self) -> bool {
        self.head == rhs.head &&
        self.tail == rhs.tail
    }
}
