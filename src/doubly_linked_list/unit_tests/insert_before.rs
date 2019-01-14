use crate::DoublyLinkedList;

#[test]
fn insert_before_on_a_list_with_one_node_yields_a_list_of_expected_length_and_order() {
    // establish
    let sample_data_1 = String::from("sample data 1");
    let sample_data_2 = String::from("sample data 2");
    let mut list = DoublyLinkedList::<String>::new();
    list.push_front(sample_data_1.clone());
    let mut iter = list.iter();

    // given a `NodeLink` referencing a `Node` in the list and new data to insert
    let node_link = iter.next().unwrap();

    // when insert_before() is called
    let _ = list.insert_before(node_link, sample_data_2.clone());

    // then the list does not report empty
    assert_eq!(list.is_empty(), false);

    // and the list reports a length of 2
    assert_eq!(list.len(), 2);

    // and the nodes yield data in the order they were inserted
    let mut iter = list.iter();
    assert_eq!(**iter.next().unwrap().borrow(), sample_data_2);
    assert_eq!(**iter.next().unwrap().borrow(), sample_data_1);

    // and after reading the expected nodes, there are no more nodes
    assert_eq!(iter.next(), None);
}
