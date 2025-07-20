use ods::singly_linked_list::SLList;
use ods::doubly_linked_list::DLList;


#[test]
fn singly_linked_list() {
    let mut list = SLList::initialize();
    list.push('a');
    list.push('b');
    list.push('c');
    assert_eq!(list.iter().collect::<Vec<&char>>(), [&'c', &'b', &'a']);
}


#[test]
fn doubly_linked_list() {
    let mut list1 = DLList::new();
    assert!(list1.get_head().is_none());
    list1.push_head(1);
    list1.push_head(2);
    list1.push_head(3);
    assert_eq!(list1.size(), 3);
    list1.push_tail(0);
    assert_eq!(*list1.get_head().unwrap(), 3);

    let mut list2 = DLList::new();
    assert!(list2.get_tail().is_none());
    list2.push_tail(-1);
    list2.push_tail(-2);
    list2.push_tail(-3);
    assert_eq!(list2.size(), 3);
    list2.push_head(0);
    assert_eq!(*list2.get_tail().unwrap(), -3);

    *list1.get_mut_head().unwrap() = 30;
    *list2.get_mut_tail().unwrap() = -30;
    assert_eq!(list1.into_iter().collect::<Vec<i32>>(), [30, 2, 1, 0]);
    assert_eq!(list2.into_iter().rev().collect::<Vec<i32>>(), [-30, -2, -1, 0]);
}
