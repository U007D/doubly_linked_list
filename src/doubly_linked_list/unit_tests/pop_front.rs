use crate::{
    DoublyLinkedList,
    Error,
};

#[test]
fn pop_front_on_an_empty_list_yields_none() {
    // given an empty list
    let expected_result = Err(Error::EmptyList);
    let mut list = DoublyLinkedList::<String>::new();

    // when pop_front() is called
    let result = list.pop_front();

    // then the list returns nothing
    assert_eq!(result, expected_result);

    // and the list reports a length of 0
    assert_eq!(list.len(), 0);
}

#[test]
fn pop_front_on_a_non_empty_list_yields_the_expected_data() {
    // given a non-empty list
    let expected_data_1 = String::from("sample test data");
    let expected_data_2 = String::from("other sample test data");
    let expected_result = Ok(expected_data_1.clone());
    let mut list = DoublyLinkedList::<String>::new();
    // no particular order
    list.push_back(expected_data_2);
    list.push_front(expected_data_1);

    // when pop_front() is called
    let result = list.pop_front();

    // then the list returns the expected data
    assert_eq!(result, expected_result);

    // and the list reports a length of 1
    assert_eq!(list.len(), 1);
}
