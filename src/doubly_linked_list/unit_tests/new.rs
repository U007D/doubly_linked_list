use super::*;

#[test]
fn new_creates_an_empty_list() {
    // given a list constructor
    let sut = DoublyLinkedList::<String>::new;

    // when invoked
    let result = sut();

    // then the list should be empty
    assert_eq!(result.is_empty(), true);
}