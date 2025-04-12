use ods::singly_linked_list::SLList;
use ods::doubly_linked_list::DLList;

#[test]
fn singly_linked_list() {
    let mut list1 = SLList::new();
    assert!(list1.is_empty());
    list1.push('a');
    list1.push('b');
    list1.push('c');
    assert_eq!(list1.iter().collect::<Vec<&char>>(), [&'c', &'b', &'a']);

    let mut list2 = SLList::new();
    list2.push('x');
    list2.push('y');
    list2.push('z');
    assert_eq!(list2.size(), 3);
    let list2_ref = list2.get_mut(2);
    assert!(list2_ref.is_some());
    *list2_ref.unwrap() = 't';
    assert_eq!(list2.iter().collect::<Vec<&char>>(), [&mut 'z', &mut 'y', &mut 't']);

    assert_eq!(list1.pop(), Some('c'));
    assert_eq!(list2.pop(), Some('z'));
    list1.append(list2);
    assert_eq!(list1.iter().collect::<Vec<&char>>(), [&'b', &'a', &'y', &'t']);

    let list3 = list1.split(1);
    assert!(list3.is_some());
    let list3 = list3.unwrap();
    assert_eq!(list1.into_iter().collect::<Vec<char>>(), ['b']);
    assert_eq!(list3.into_iter().collect::<Vec<char>>(), ['a', 'y', 't']);
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

    assert_eq!(list1.into_iter().collect::<Vec<i32>>(), [3, 2, 1, 0]);
    assert_eq!(list2.into_iter().rev().collect::<Vec<i32>>(), [-3, -2, -1, 0]);
}