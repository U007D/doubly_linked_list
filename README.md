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
* `Drop`ing extremely long lists could cause a stack overflow, as the `drop()` of `Node` n makes a call to drop to 
`Node` n + 1, which cascades all the way to the tail of the list.  The fix for this is to implement `Drop` for `Node`
 such that it walks the list from the tail (or first subsequent node which has any additional outstanding live 
 references--whichever is encountered first) and `Drop`s the nodes in reverse order to avoid the build-up of stack 
 calls.  This is straightforward to do because this list is doubly-linked--it's clear how to iterate back toward the 
 starting node without doubling memory usage or entering O(n^2) territory.
* Ergonomics of iterator is really awful--difficult to hide because of the way `RefCell::borrow()`/`borrow_mut()` works
* Via internal mutation, it is possible to iterate to a `Node` via `DoublyLinkedList::iter()` and then 
use `doubly_linked_list::Iter::borrow_mut()` to modify the `Node`'s value.  This is unidiomatic and is a result of 
not encapsulating the `RefCell::borrow()` inside `doubly_linked_list::Iter` itself.
* Iterating the `DoublyLinkedList` yields `NodeLink`s; accessing the `Node` represented by a `NodeLink` (via 
`RefCell::borrow()` or `RefCell::borrow_mut()`) and holding on to the borrow while otherwise mutating the node (e.g. 
modifying adjacent nodes) will (correctly) cause a runtime `borrowck` violation.  Ergonomically, this is not ideal, but 
is a limitation of the current `RefCell` implementation at `Node`-level granularity (as opposed to use of `Atomic`s or field-level 
granularity).

### Production implementation:
* If building an actual DoublyLinkedList for production, I recommend using a vector of nodes as storage (possibly 
linked if avoiding expensive reallocations is required).  To create a lock-free implementation at the (worst-case) cost
 of doubling memory usage, back the store with a double-buffered write cache (similar to https://youtu.be/s19G6n0UjsM).
