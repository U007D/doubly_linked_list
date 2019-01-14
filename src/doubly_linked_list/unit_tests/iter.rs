use super::*;

#[test]
fn iter_from_empty_list_returns_expected_iterator() {
    // given an iterator over an empty list
    let expected_result = Iter(None);
    let list = DoublyLinkedList::<String>::new();

    // when .iter() is called
    let result = list.iter();

    // then the iterator should return `None`
    assert_eq!(result, expected_result);
}
