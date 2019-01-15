## Doubly Linked List exercise

This is an implementation of a pretty terrible data structure, the linked list.  This implementation is in 
Rust as an exercise.  It was built subject to the following constraints:
* conforms to the Rust Ownership model, as enforced by `borrowck` (i.e. no `unsafe`, no manual lifetime management 
via array/vector-backed indices).
* node level resolution (no locking of entire List for mutation)
* O(1) insert performance
* Doubly-linked (bi-directional) for extra fun and profit!
* Iterable, as one would expect of a collection

### Known Issues:
* Did I mention that linked lists are really terrible data structures?  (Poor locality).
* Ergonomics of iterator is really awful--difficult to hide because of the way `RefCell::borrow()`/`borrow_mut()` works
* Via internal mutation, it is possible to iterate to a `Node` via `DoublyLinkedList::iter()` and then 
use `doubly_linked_list::Iter::borrow_mut()` to modify the `Node`'s value.  This is unidiomatic and is a result of 
not encapsulating the `RefCell::borrow()` inside `doubly_linked_list::Iter` itself.
* Iterating the `DoublyLinkedList` yields `NodeLink`s; accessing the `Node` represented by a `NodeLink` (via 
`RefCell::borrow()` or `RefCell::borrow_mut()`) and holding on to the borrow while otherwise mutating the node (e.g. 
modifying adjacent nodes) will create a runtime `borrowck` violation.  This is not ideal, but is a limitation of the 
current `RefCell` implementation at `Node`-level granularity (as opposed to use of `Atomic`s or field-level 
granularity).
