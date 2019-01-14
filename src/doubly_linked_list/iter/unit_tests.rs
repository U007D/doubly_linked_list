#![allow(clippy::option_unwrap_used)]
use crate::{
    DoublyLinkedList,
};

#[test]
fn iter_from_empty_list_returns_iterator_yielding_none() {
    // establish
    let list = DoublyLinkedList::<String>::new();

    // given an iterator over an empty list
    let mut sut = list.iter();

    // when the iterator is called once
    let result = sut.next();

    // then the iterator should return `None`
    assert_eq!(result, None);
}

#[test]
fn iter_from_non_empty_list_returns_iterator_yielding_some_t() {
    // establish
    let expected_data = String::from("Hello");
    let mut list = DoublyLinkedList::<String>::new();
    list.push_back(expected_data.clone());

    // given an iterator over a non-empty list
    let mut sut = list.iter();

    // when the iterator is called once
    let result_1 = sut.next();

    // then the iterator should return the expected data
    assert_eq!(**result_1.unwrap().borrow(), expected_data);

    // and when the iterator is called a second time
    let result_2 = sut.next();

    // then the iterator should return `None`
    assert_eq!(result_2, None);
}
