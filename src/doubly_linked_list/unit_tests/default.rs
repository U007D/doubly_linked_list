use super::*;

#[test]
fn default_creates_the_same_list_as_new() {
    // given a list constructor
    let expected_list = DoublyLinkedList::<String>::new();
    let sut = DoublyLinkedList::<String>::default;

    // when invoked
    let result = sut();

    // then the list should be empty
    assert_eq!(result, expected_list);
}