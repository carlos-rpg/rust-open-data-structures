use ods::singly_linked_list::SLList;

#[test]
fn push_add_alternating() {
    let mut list = SLList::new();
    list.push(0);
    list.add(-1);
    list.push(1);
    list.add(-2);
    list.push(2);
    list.add(-3);
    assert_eq!(list.size(), 6);
    assert_eq!(list.into_iter().collect::<Vec<i32>>(), [2, 1, 0, -1, -2, -3])
}

#[test]
fn add_push_alternating() {
    let mut list = SLList::new();
    list.add(0);
    list.push(-1);
    list.add(1);
    list.push(-2);
    list.add(2);
    list.push(-3);
    assert_eq!(list.size(), 6);
    assert_eq!(list.into_iter().collect::<Vec<i32>>(), [-3, -2, -1, 0, 1, 2])
}

#[test]
fn push_then_pop_until_empty_then_add_then_pop_until_empty() {
    let mut list = SLList::new();
    list.push('a');
    list.push('b');
    assert_eq!(list.size(), 2);
    assert_eq!(list.pop(), Some('b'));
    assert_eq!(list.pop(), Some('a'));
    assert_eq!(list.pop(), None);
    assert_eq!(list.size(), 0);
    list.add('x');
    list.add('y');
    assert_eq!(list.size(), 2);
    assert_eq!(list.pop(), Some('x'));
    assert_eq!(list.pop(), Some('y'));
    assert_eq!(list.pop(), None);
    assert_eq!(list.size(), 0);
    assert_eq!(list.into_iter().collect::<Vec<char>>(), []);
}
