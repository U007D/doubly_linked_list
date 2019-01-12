use super::*;

#[test]
fn new_node_has_expected_state() {
    // given a `Node` constructor
    let expected_data = String::from("sample node data");
    let sut = Node::<String>::new;

    // when invoked
    let result = sut(expected_data.clone());

    // then the node should contain the expected data
    assert_eq!(*result, expected_data);
    assert_eq!(*result, expected_data);

    // and the `next` link should be `None`
    assert_eq!(result.next, None);

    // and the `prev` link should be `None`
    assert_eq!(result.prev, None);
}