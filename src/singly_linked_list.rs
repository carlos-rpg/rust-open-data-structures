//! A safe singly linked list with head access.
//! 
//! Unlike the book, this implementation avoids the tail reference on purpose 
//! because single linked lists are ideal to show what the `Box` smart pointer 
//! can do. Although limited to one owner like any other mutable reference, `Box` 
//! is much more flexible than `Rc<RefCell<>>` used for the doubly linked list.


/// A safe singly linked list.
pub struct SLList<T> {
    head: Option<Box<Node<T>>>,
    size: usize,
}


struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    fn new(value: T, next: Option<Box<Node<T>>>) -> Box<Node<T>> {
        Box::new(Self { value, next })
    }
}


impl<T> SLList<T> {
    /// Creates a new, empty list.
    /// 
    /// # Examples
    /// 
    /// ```
    /// # use ods::singly_linked_list::SLList;
    /// let list: SLList<i32> = SLList::new();
    /// ```
    pub fn new() -> Self {
        Self { head: None, size: 0 }
    }

    /// Returns the number of element in the list.
    /// 
    /// # Examples
    /// 
    /// ```
    /// # use ods::singly_linked_list::SLList;
    /// let list: SLList<i32> = SLList::new();
    /// assert_eq!(list.size(), 0);
    /// ```
    pub fn size(&self) -> usize {
        self.size
    }

    /// Returns true if the list is empty, false otherwise.
    /// 
    /// # Examples
    /// 
    /// ```
    /// # use ods::singly_linked_list::SLList;
    /// let list: SLList<i32> = SLList::new();
    /// assert!(list.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    /// Returns a shared reference to the value at the given position.
    /// 
    /// # Examples
    /// 
    /// ```
    /// # use ods::singly_linked_list::SLList;
    /// let mut list = SLList::new();
    /// list.push('a');
    /// assert_eq!(list.get(0), Some(&'a'));
    /// ```
    pub fn get(&self, at: usize) -> Option<&T> {
        if at >= self.size() {
            return None;
        }
        let mut node = self.head
            .as_deref()
            .expect("`at` should be < than `self.size()`"); 

        for _ in 0..at {
            node = node.next
                .as_deref()
                .expect("`at` should be < than `self.size()`"); 
        }
        Some(&node.value)
    }

    /// Returns a mutable reference to the value at the given position. Returns 
    /// none if the position is out of the list.
    /// 
    /// # Examples
    /// 
    /// ```
    /// # use ods::singly_linked_list::SLList;
    /// let mut list = SLList::new();
    /// list.push('a');
    /// let mut mutable = list.get_mut(0).unwrap();
    /// assert_eq!(*mutable, 'a');
    /// *mutable = 'x';
    /// assert_eq!(*mutable, 'x');
    /// ```
    pub fn get_mut(&mut self, at: usize) -> Option<&mut T> {
        if at >= self.size() {
            return None;
        }
        let mut node = self.head
            .as_deref_mut()
            .expect("`at` should be < than `self.size()`");

        for _ in 0..at {
            node = node.next
                .as_deref_mut()
                .expect("`at` should be < than `self.size()`");
        }
        Some(&mut node.value)
    }

    /// Inserts a value as the new head of the list.
    /// 
    /// # Examples
    /// 
    /// ```
    /// # use ods::singly_linked_list::SLList;
    /// let mut list = SLList::new();
    /// list.push('a');
    /// assert_eq!(list.get(0), Some(&'a'));
    /// list.push('b');
    /// assert_eq!(list.get(0), Some(&'b'));
    /// ```
    pub fn push(&mut self, x: T) {
        self.head = Some(Node::new(x, self.head.take()));
        self.size += 1;
    }

    /// Removes the value at the head of the list and returns it. Returns None 
    /// if the list is empty.
    /// 
    /// # Examples
    /// 
    /// ```
    /// # use ods::singly_linked_list::SLList;
    /// let mut list = SLList::new();
    /// list.push('a');
    /// list.push('b');
    /// assert_eq!(list.pop(), Some('b'));
    /// assert_eq!(list.pop(), Some('a'));
    /// ```
    pub fn pop(&mut self) -> Option<T> {
        let mut pop_node = self.head.take()?;
        self.head = pop_node.next.take();
        self.size -= 1;
        Some(pop_node.value)
    }

    /// Returns an iterator of shared references to the list's values.
    /// 
    /// # Examples
    /// 
    /// ```
    /// # use ods::singly_linked_list::SLList;
    /// let mut list = SLList::new();
    /// list.push('a');
    /// assert_eq!(list.iter().next(), Some(&'a'));
    /// ```
    pub fn iter(&self) -> Iter<T> {
        Iter(self.head.as_ref())
    }

    /// Returns an iterator of mutable references to the list's values.
    /// 
    /// # Examples
    /// 
    /// ```
    /// # use ods::singly_linked_list::SLList;
    /// let mut list = SLList::new();
    /// list.push('a');
    /// assert_eq!(list.iter_mut().next(), Some(&mut 'a'));
    /// ```
    pub fn iter_mut(&mut self) -> IterMut<T> {
        IterMut(self.head.as_mut())
    }

    /// Splits `self` at the given position, returning the rest of the list as 
    /// a new one. Returns None if the position is out of the list.
    /// 
    /// # Examples
    /// 
    /// ```
    /// # use ods::singly_linked_list::SLList;
    /// let mut list1 = SLList::new();
    /// list1.push('a');
    /// list1.push('b');
    /// list1.push('c');
    /// let mut list2 = list1.split(1).unwrap();
    /// assert_eq!(list1.into_iter().collect::<Vec<char>>(), ['c']);
    /// assert_eq!(list2.into_iter().collect::<Vec<char>>(), ['b', 'a']);
    /// ```
    pub fn split(&mut self, at: usize) -> Option<Self> {
        if at > self.size() {
            return None;
        }
        let mut node_opt = &mut self.head;
        for _ in 0..at {
            node_opt = &mut node_opt
                .as_mut()
                .expect("`at` should be <= than `self.size()`")
                .next;
        }
        let other = Self { 
            head: node_opt.take(),
            size: self.size() - at,
        };
        self.size = at;
        Some(other)
    }
    
    /// Appends `other` to the end of `self`.
    /// 
    /// # Examples
    /// 
    /// ```
    /// # use ods::singly_linked_list::SLList;
    /// let mut list1 = SLList::new();
    /// list1.push('a');
    /// list1.push('b');
    /// let mut list2 = SLList::new();
    /// list2.push('c');
    /// list2.push('d');
    /// list1.append(list2);
    /// assert_eq!(list1.into_iter().collect::<Vec<char>>(), ['b', 'a', 'd', 'c']);
    /// ```
    pub fn append(&mut self, mut other: Self) {
        let mut node_opt = &mut self.head;

        while let Some(node) = node_opt {
            node_opt = &mut node.next;
        }
        *node_opt = other.head.take();
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


pub struct Iter<'a, T>(Option<&'a Box<Node<T>>>);

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let node = self.0?;
        self.0 = node.next.as_ref();
        Some(&node.value)
    }
}


pub struct IterMut<'a, T>(Option<&'a mut Box<Node<T>>>);

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        let node = self.0.take()?;
        self.0 = node.next.as_mut();
        Some(&mut node.value)
    }
}


impl<T> Drop for SLList<T> {
    fn drop(&mut self) {
        // The reason for this custom implementation is that the default one is 
        // recursive, which has the risk of blowing the stack if the list is large 
        // enough.
        let mut node_opt = self.head.take();
        while let Some(mut node) = node_opt {
            node_opt = node.next.take();
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    fn build_test_list() -> SLList<i32> {
        let node2 = Box::new(Node { value: 2, next: None });
        let node1 = Box::new(Node { value: 1, next: Some(node2) });
        let node0 = Box::new(Node { value: 0, next: Some(node1) });
        SLList { head: Some(node0), size: 3 }
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
