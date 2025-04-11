//! A safe singly linked list with head access


pub struct SLList<T> {
    head: Option<Link<T>>,
    size: usize,
}

type Link<T> = Box<Node<T>>;

struct Node<T> {
    value: T,
    next: Option<Link<T>>,
}

impl<T> Node<T> {
    fn new(value: T, next: Option<Link<T>>) -> Link<T> {
        Box::new(Self { value, next })
    }
}


impl<T> SLList<T> {
    pub fn new() -> Self {
        Self { head: None, size: 0 }
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn get(&self, at: usize) -> Option<&T> {
        let mut node = self.head.as_deref()?; 
        for _ in 0..at {
            node = node.next.as_deref()?;
        }
        Some(&node.value)
    }

    pub fn get_mut(&mut self, at: usize) -> Option<&mut T> {
        let mut node = self.head.as_deref_mut()?;
        for _ in 0..at {
            node = node.next.as_deref_mut()?;
        }
        Some(&mut node.value)
    }

    pub fn push(&mut self, x: T) {
        self.head = Some(Node::new(x, self.head.take()));
        self.size += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        let mut pop_link = self.head.take()?;
        self.head = pop_link.next.take();
        self.size -= 1;
        Some(pop_link.value)
    }

    pub fn iter(&self) -> Iter<T> {
        Iter(self.head.as_ref())
    }

    pub fn iter_mut(&mut self) -> IterMut<T> {
        IterMut(self.head.as_mut())
    }

    pub fn split(&mut self, at: usize) -> Option<Self> {
        let mut other = Self::new();

        other.head = if at == 0 {
            self.head.take()
        } else {
            let mut link = self.head.as_mut()?;
            for _ in 1..at {
                link = link.next.as_mut()?;
            }
            link.next.take()
        };
        other.size = self.size() - at;
        self.size = at;
        Some(other)
    }

    pub fn append(&mut self, mut other: Self) {
        let self_size = self.size();

        if self_size == 0 {
            self.head = other.head.take();
        } else {
            let mut link = self.head
                .as_mut()
                .expect("`self.head` should be `Some(_)`");

            for _ in 1..self_size {
                link = link.next
                    .as_mut()
                    .expect("`link.next` should be `Some(_)`");
            }
            link.next = other.head.take();
        }
        self.size += other.size();
    }
}


impl<T> IntoIterator for SLList<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter(self)
    }
}


pub struct IntoIter<T>(SLList<T>);

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}


pub struct Iter<'a, T>(Option<&'a Link<T>>);

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let link = self.0?;
        self.0 = link.next.as_ref();
        Some(&link.value)
    }
}


pub struct IterMut<'a, T>(Option<&'a mut Link<T>>);

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        let link = self.0.take()?;
        self.0 = link.next.as_mut();
        Some(&mut link.value)
    }
}


impl<T> Drop for SLList<T> {
    fn drop(&mut self) {
        // The reason for this custom implementation is that the default one is 
        // recursive, which has the risk of blowing the stack if the list is large 
        // enough.
        let mut link_opt = self.head.take();
        while let Some(mut link) = link_opt {
            link_opt = link.next.take();
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    fn build_test_list() -> SLList<i32> {
        let link2 = Box::new(Node { value: 2, next: None });
        let link1 = Box::new(Node { value: 1, next: Some(link2) });
        let link0 = Box::new(Node { value: 0, next: Some(link1) });
        SLList { head: Some(link0), size: 3 }
    }

    #[test]
    fn iter_empty_list_returns_nothing() {
        let list: SLList<i32> = SLList { head: None, size: 0 };
        let mut list_iter = list.iter();
        assert_eq!(list_iter.next(), None);
        assert_eq!(list_iter.next(), None);
    }

    #[test]
    fn iter_non_empty_list_returns_references_to_contents() {
        let list = build_test_list();
        let mut list_iter = list.iter();
        assert_eq!(list_iter.next(), Some(&0));
        assert_eq!(list_iter.next(), Some(&1));
        assert_eq!(list_iter.next(), Some(&2));
        assert_eq!(list_iter.next(), None);
        assert_eq!(list_iter.next(), None);
    }

    #[test]
    fn iter_mut_empty_list_returns_nothing() {
        let mut list: SLList<i32> = SLList { head: None, size: 0 };
        let mut list_iter = list.iter_mut();
        assert_eq!(list_iter.next(), None);
        assert_eq!(list_iter.next(), None);
    }

    #[test]
    fn iter_mut_non_empty_list_returns_mutable_references_to_contents() {
        let mut list = build_test_list();
        let mut list_iter = list.iter_mut();
        assert_eq!(list_iter.next(), Some(&mut 0));
        assert_eq!(list_iter.next(), Some(&mut 1));
        assert_eq!(list_iter.next(), Some(&mut 2));
        assert_eq!(list_iter.next(), None);
        assert_eq!(list_iter.next(), None);
    }

    #[test]
    fn get_size_zero_returns_none() {
        let list: SLList<i32> = SLList { head: None, size: 0 };
        assert_eq!(list.get(0), None);
        assert_eq!(list.get(1), None);
        assert_eq!(list.get(2), None);
    }

    #[test]
    fn get_size_one_returns_some_and_none() {
        let list = SLList { 
            head: Some(Box::new(Node {value: 'x', next: None })),
            size: 1, 
        };
        assert_eq!(list.get(0), Some(&'x'));
        assert_eq!(list.get(1), None);
        assert_eq!(list.get(2), None);
    }

    #[test]
    fn get_size_greater_than_one_returns_some_and_none() {
        let list = build_test_list();
        assert_eq!(list.get(0), Some(&0));
        assert_eq!(list.get(1), Some(&1));
        assert_eq!(list.get(2), Some(&2));
        assert_eq!(list.get(3), None);
        assert_eq!(list.get(4), None);
    }

    #[test]
    fn get_mut_size_one_mutates_list() {
        let mut list = SLList { 
            head: Some(Box::new(Node {value: 0, next: None })),
            size: 1, 
        };
        let zero = list.get_mut(0).unwrap();
        assert_eq!(zero, &0);
        *zero = 1;
        assert_eq!(zero, &1);
    }

    #[test]
    fn pop_returns_values() {
        let mut list = build_test_list();
        assert_eq!(list.pop(), Some(0));
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), Some(2));
        assert_eq!(list.pop(), None);
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn pop_keeps_track_of_size() {
        let mut list = build_test_list();
        list.pop();
        assert_eq!(list.size(), 2);
        list.pop();
        assert_eq!(list.size(), 1);
        list.pop();
        assert_eq!(list.size(), 0);
        list.pop();
        assert_eq!(list.size(), 0);
    }

    #[test]
    fn push_grows_the_list() {
        let mut list = SLList { head: None, size: 0 };
        list.push(0);
        list.push(1);
        list.push(2);
        list.push(3);
        let mut list_iter = list.iter();
        assert_eq!(list_iter.next(), Some(&3));
        assert_eq!(list_iter.next(), Some(&2));
        assert_eq!(list_iter.next(), Some(&1));
        assert_eq!(list_iter.next(), Some(&0));
    }

    #[test]
    fn push_keeps_track_of_size() {
        let mut list = SLList { head: None, size: 0 };
        list.push(0);
        assert_eq!(list.size(), 1);
        list.push(1);
        assert_eq!(list.size(), 2);
        list.push(2);
        assert_eq!(list.size(), 3);
        list.push(3);
        assert_eq!(list.size(), 4);
    }

    #[test]
    fn split_empty_list_returns_empty_list() {
        let mut list1: SLList<i32> = SLList { head: None, size: 0 };
        let list2 = list1.split(0).unwrap();
        assert_eq!(list1.iter().next(), None);
        assert_eq!(list2.iter().next(), None);
    }

    #[test]
    fn split_empty_list_returns_sizes_as_zero() {
        let mut list1: SLList<i32> = SLList { head: None, size: 0 };
        let list2 = list1.split(0).unwrap();
        assert_eq!(list1.size(), 0);
        assert_eq!(list2.size(), 0);
    }

    #[test]
    fn split_at_zero_returns_full_original_list() {
        let mut list1 = build_test_list();
        let list2 = list1.split(0).unwrap();
        assert_eq!(list1.into_iter().next(), None);
        let mut list2_iter = list2.iter();
        assert_eq!(list2_iter.next(), Some(&0));
        assert_eq!(list2_iter.next(), Some(&1));
        assert_eq!(list2_iter.next(), Some(&2));
    }

    #[test]
    fn split_at_zero_returns_zero_and_full_sizes() {
        let mut list1 = build_test_list();
        let list2 = list1.split(0).unwrap();
        assert_eq!(list1.size(), 0);
        assert_eq!(list2.size(), 3);
    }

    #[test]
    fn split_at_end_returns_empty_list() {
        let mut list1 = build_test_list();
        let list2 = list1.split(list1.size()).unwrap();
        let mut list1_iter = list1.iter();
        assert_eq!(list1_iter.next(), Some(&0));
        assert_eq!(list1_iter.next(), Some(&1));
        assert_eq!(list1_iter.next(), Some(&2));
        assert_eq!(list2.iter().next(), None);
    }

    #[test]
    fn split_at_end_returns_full_and_zero_sizes() {
        let mut list1 = build_test_list();
        let list2 = list1.split(list1.size()).unwrap();
        assert_eq!(list1.size(), 3);
        assert_eq!(list2.size(), 0);
    }

    #[test]
    fn split_at_mid_returns_partial_list() {
        let mut list1 = build_test_list();
        let list2 = list1.split(1).unwrap();
        let mut list1_iter = list1.iter();
        assert_eq!(list1_iter.next(), Some(&0));
        let mut list2_iter = list2.iter();
        assert_eq!(list2_iter.next(), Some(&1));
        assert_eq!(list2_iter.next(), Some(&2));
    }

    #[test]
    fn split_at_mid_returns_partial_sizes() {
        let mut list1 = build_test_list();
        let list2 = list1.split(1).unwrap();
        assert_eq!(list1.size(), 1);
        assert_eq!(list2.size(), 2);
    }

    #[test]
    fn split_out_of_range_returns_none() {
        let mut list = build_test_list();
        assert!(list.split(4).is_none());
        assert!(list.split(5).is_none());
        assert!(list.split(4).is_none());
    }

    #[test]
    fn split_out_of_range_doesnt_change_size() {
        let mut list = build_test_list();
        assert_eq!(list.size(), 3);
        list.split(4);
        assert_eq!(list.size(), 3);
        list.split(5);
        assert_eq!(list.size(), 3);
        list.split(4);
        assert_eq!(list.size(), 3);
    }

    #[test]
    fn append_empty_list_to_empty_list_stays_empty() {
        let mut list1: SLList<i32> = SLList { head: None, size: 0 };
        let list2 = SLList { head: None, size: 0 };
        list1.append(list2);
        assert_eq!(list1.into_iter().collect::<Vec<i32>>(), []);
    }

    #[test]
    fn append_emtpy_to_non_empty_stays_unchanged() {
        let mut list1 = build_test_list();
        let list2 = SLList { head: None, size: 0};
        list1.append(list2);
        let mut list1_iter = list1.iter();
        assert_eq!(list1_iter.next(), Some(&0));
        assert_eq!(list1_iter.next(), Some(&1));
        assert_eq!(list1_iter.next(), Some(&2));
    }

    #[test]
    fn append_non_emtpy_to_empty_transfers_contents() {
        let mut list1 = SLList { head: None, size: 0 };
        let list2 = build_test_list();
        list1.append(list2);
        let mut list1_iter = list1.iter();
        assert_eq!(list1_iter.next(), Some(&0));
        assert_eq!(list1_iter.next(), Some(&1));
        assert_eq!(list1_iter.next(), Some(&2));
    }

    #[test]
    fn append_non_emtpy_to_non_empty_transfers_contents() {
        let mut list1 = build_test_list();
        let list2 = build_test_list();
        list1.append(list2);
        let mut list1_iter = list1.iter();
        assert_eq!(list1_iter.next(), Some(&0));
        assert_eq!(list1_iter.next(), Some(&1));
        assert_eq!(list1_iter.next(), Some(&2));
        assert_eq!(list1_iter.next(), Some(&0));
        assert_eq!(list1_iter.next(), Some(&1));
        assert_eq!(list1_iter.next(), Some(&2));
    }

    #[test]
    fn append_emtpy_to_non_empty_keeps_size() {
        let mut list1 = build_test_list();
        let list2 = SLList { head: None, size: 0};
        list1.append(list2);
        assert_eq!(list1.size(), 3);
    }

    #[test]
    fn append_non_emtpy_to_empty_updates_size() {
        let mut list1 = SLList { head: None, size: 0 };
        let list2 = build_test_list();
        list1.append(list2);
        assert_eq!(list1.size(), 3);
    }

    #[test]
    fn append_non_emtpy_to_non_empty_updates_size() {
        let mut list1 = build_test_list();
        let list2 = build_test_list();
        list1.append(list2);
        assert_eq!(list1.size(), 6);
    }
}
