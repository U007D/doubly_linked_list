#[cfg(test)]
mod unit_tests;
use crate::{
    Node,
    NodeLink,
    WeakLink,
};

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

    pub fn len(&self) -> usize {
        let mut count = 0;
        let mut link_opt = self.head.clone();
        while let Some(link) = link_opt {
            link_opt = link.borrow().next.clone();
            count += 1;
        }
        count
    }

    pub fn push_back(&mut self, node: Node<T>) -> &mut Self {
        let node_link = NodeLink::new(node);
        let prev_tail = self.tail.take();
        self.tail = Some(node_link.to_weak());
        match prev_tail {
            Some(prev) => prev.upgrade()
                              .expect("Internal error: DoublyLinkedList impl is deleting .next before .prev")
                              .borrow_mut()
                              .next = Some(node_link),
            None => self.head = Some(node_link),
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
