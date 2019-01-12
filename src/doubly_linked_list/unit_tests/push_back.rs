use crate::{
    DoublyLinkedList,
    Node,
};

#[test]
fn push_back_on_an_empty_list_yields_non_empty_list_of_expected_length() {
    // given an empty list and a node to append
    let sample_data = String::from("sample node data");
    let node = Node::new(sample_data.clone());
    let mut list = DoublyLinkedList::<String>::new();

    // when push_back() is called
    let _result = list.push_back(node);

    // then the list does not report empty
    assert_eq!(list.is_empty(), false);

    // and the list reports a length of 1
    assert_eq!(list.len(), 1);
}
