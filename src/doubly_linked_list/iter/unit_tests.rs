use crate::DoublyLinkedList;

#[test]
fn iter_from_empty_list_returns_iterator_yielding_none() {
    // given an iterator over an empty list
    let list = DoublyLinkedList::<String>::new();
    let mut sut = list.iter();

    // when the iterator is called once
    let result = sut.next();

    // then the iterator should return `None`
    assert_eq!(result, Option::<&String>::None);
}

#[test]
fn iter_from_non_empty_list_returns_iterator_yielding_some_t() {
    // given an iterator over a non-empty list
    let expected_data = String::from("Hello");
    let mut list = DoublyLinkedList::<String>::new();
    list.push_back(expected_data.clone());
    let mut sut = list.iter();

    // when the iterator is called once
    let result1 = sut.next();

    // then the iterator should return the expected data
    assert_eq!(result1, Some(&expected_data));

    // when the iterator is called a second time
    let result2 = sut.next();

    // then the iterator should return `None`
    assert_eq!(result2, Option::<&String>::None);
}
