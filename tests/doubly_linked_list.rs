use ods::doubly_linked_list::DLList;

#[test]
fn push_head_and_tail_alternating_collect_from_head() {
    let mut list = DLList::new();
    list.push_head(0);
    list.push_tail(-1);
    list.push_head(1);
    list.push_tail(-2);
    list.push_head(2);
    list.push_tail(-3);
    assert_eq!(list.size(), 6);
    assert_eq!(list.into_iter().collect::<Vec<i32>>(), [2, 1, 0, -1, -2, -3])
}

#[test]
fn push_tail_then_head_collect_from_tail() {
    let mut list = DLList::new();
    list.push_tail(-1);
    list.push_tail(-2);
    list.push_tail(-3);
    list.push_head(0);
    list.push_head(1);
    list.push_head(2);
    assert_eq!(list.size(), 6);
    assert_eq!(list.into_iter().rev().collect::<Vec<i32>>(), [-3, -2, -1, 0, 1, 2])
}

#[test]
fn push_then_pop_until_empty_then_add_then_pop_until_empty() {
    let mut list = DLList::new();
    list.push_head('a');
    list.push_head('b');
    assert_eq!(list.size(), 2);
    assert_eq!(list.pop_head(), Some('b'));
    assert_eq!(list.pop_head(), Some('a'));
    assert_eq!(list.pop_head(), None);
    assert_eq!(list.size(), 0);
    list.push_tail('x');
    list.push_tail('y');
    assert_eq!(list.size(), 2);
    assert_eq!(list.pop_tail(), Some('y'));
    assert_eq!(list.pop_tail(), Some('x'));
    assert_eq!(list.pop_tail(), None);
    assert_eq!(list.size(), 0);
    assert_eq!(list.into_iter().collect::<Vec<char>>(), []);
}

#[test]
fn push_and_get_references_to_values() {
    let mut list = DLList::new();
    assert!(list.get_head().is_none());
    assert!(list.get_tail().is_none());
    list.push_head('a');
    assert_eq!(*list.get_head().unwrap(), 'a');
    assert_eq!(*list.get_tail().unwrap(), 'a');
    list.push_tail('x');
    assert_eq!(*list.get_head().unwrap(), 'a');
    assert_eq!(*list.get_tail().unwrap(), 'x');
    list.push_head('b');
    assert_eq!(*list.get_head().unwrap(), 'b');
    assert_eq!(*list.get_tail().unwrap(), 'x');
}
