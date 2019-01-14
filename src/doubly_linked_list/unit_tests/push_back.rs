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
    let result = list.push_back(node);

    // then the list does not report empty
    assert_eq!(result.is_empty(), false);

    // and the list reports a length of 1
    assert_eq!(list.len(), 1);
}

#[test]
fn push_back_on_a_list_with_one_node_yields_a_list_of_expected_length_and_order() {
    // given a list with one node and a node to append
    let sample_data1 = String::from("sample data 1");
    let sample_data2 = String::from("sample data 2");
    let mut list = DoublyLinkedList::<String>::new();
    list.push_back(Node::new(sample_data1.clone()));

    // when push_back() is called
    let _ = list.push_back(Node::new(sample_data2.clone()));

    // then the list does not report empty
    assert_eq!(list.is_empty(), false);

    // and the list reports a length of 2
    assert_eq!(list.len(), 2);

    // and the nodes yield data in the order they were inserted
    let mut iter = list.iter();
    assert_eq!(iter.next(), Some(&sample_data1));
    assert_eq!(iter.next(), Some(&sample_data2));

    // and after reading the expected nodes, there are no more nodes
    assert_eq!(iter.next(), None);
}

#[test]
fn push_back_on_a_list_with_one_node_in_a_different_order_yields_a_list_of_expected_length_and_order() {
    // given a list with one node and a node to append
    let sample_data1 = String::from("sample data 1");
    let sample_data2 = String::from("sample data 2");
    let mut list = DoublyLinkedList::<String>::new();
    list.push_back(Node::new(sample_data2.clone()));

    // when push_back() is called
    let _ = list.push_back(Node::new(sample_data1.clone()));

    // then the list does not report empty
    assert_eq!(list.is_empty(), false);

    // and the list reports a length of 2
    assert_eq!(list.len(), 2);

    // and the nodes yield data in the order they were inserted
    let mut iter = list.iter();
    assert_eq!(iter.next(), Some(&sample_data2));
    assert_eq!(iter.next(), Some(&sample_data1));

    // and after reading the expected nodes, there are no more nodes
    assert_eq!(iter.next(), None);
}