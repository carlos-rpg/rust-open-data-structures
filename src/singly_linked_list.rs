//! A safe singly linked list with head and tail access.

use std::rc::Rc;
use std::cell::RefCell;

type Link<T> = Rc<RefCell<Node<T>>>;

/// A safe singly linked list.
/// 
/// This implementation allows insertion at the head and the tail of the list 
/// in constant time, but only extraction from the head is constant.
/// 
/// # Panics
/// 
/// Only anomalous states should trigger a panic, such as a head with more than 
/// one shared owner or a head being Some(_) but not the tail and viceversa.
///
/// # Examples
/// 
/// ```
/// # use ods::singly_linked_list::SLList;
/// let mut list = SLList::new();
/// list.push(1);
/// list.add(-1);
/// assert_eq!(list.pop(), Some(1));
/// assert_eq!(list.pop(), Some(-1));
/// ```
pub struct SLList<T> {
    head: Option<Link<T>>,
    tail: Option<Link<T>>,
    size: usize,
}

struct Node<T> {
    value: T,
    next: Option<Link<T>>,
}

impl<T> Node<T> {
    fn new(value: T, next: Option<&Link<T>>) -> Link<T> {
        Rc::new(RefCell::new(
            Self { value, next: next.map(|link| Rc::clone(link)) }
        ))
    }
}

impl<T> SLList<T> {
    /// Creates a new, empty list.
    /// 
    /// # Examples
    /// 
    /// ```
    /// # use ods::singly_linked_list::SLList;
    /// let list: SLList<bool> = SLList::new();
    /// assert_eq!(list.size(), 0);
    /// ```
    pub fn new() -> Self {
        Self { head: None, tail: None, size: 0 }
    }

    /// Returns the number of elements currently in the list.
    /// 
    /// # Examples
    /// 
    /// ```
    /// # use ods::singly_linked_list::SLList;
    /// let mut list = SLList::new();
    /// list.push('a');
    /// assert_eq!(list.size(), 1);
    /// ```
    pub fn size(&self) -> usize {
        self.size
    }

    /// Inserts an element at the head of the list.
    /// 
    /// # Examples
    /// 
    /// ```
    /// # use ods::singly_linked_list::SLList;
    /// let mut list = SLList::new();
    /// list.push(0);
    /// ```
    pub fn push(&mut self, x: T) {
        let new_link = Node::new(x, self.head.as_ref());
        self.head = Some(Rc::clone(&new_link));

        if self.tail.is_none() {
            self.tail = Some(new_link);
        }
        self.size += 1;
    }

    /// Extracts the element at the head of the list.
    /// 
    /// # Panics
    /// 
    /// If the head of the list had a strong count for shared ownership greater 
    /// than one, this would panic. This is however something that under a 
    /// correct implementation should never happen.
    /// 
    /// # Examples
    /// 
    /// ```
    /// # use ods::singly_linked_list::SLList;
    /// let mut list = SLList::new();
    /// list.push(0);
    /// assert_eq!(list.pop(), Some(0));
    /// ```
    pub fn pop(&mut self) -> Option<T> {
        let pop_link = Rc::clone(self.head.as_ref()?);
        self.head = pop_link.borrow_mut().next.take();

        if self.head.is_none() {
            self.tail = None;
        }
        let pop_contents = Rc::into_inner(pop_link)
            .expect("`pop_link` strong count should be 1")
            .into_inner();

        self.size -= 1;
        Some(pop_contents.value)
    }

    /// Inserts an element at the tail of the list.
    /// 
    /// # Panics
    /// 
    /// If the head is `Some(_)` but the tail is `None`, the function panics. 
    /// But this should never happen under a correct implementation.
    /// 
    /// # Examples
    /// 
    /// ```
    /// # use ods::singly_linked_list::SLList;
    /// let mut list = SLList::new();
    /// list.add(10);
    /// assert_eq!(list.pop(), Some(10));
    /// ```
    pub fn add(&mut self, x: T) {
        let new_link = Node::new(x, None);

        match self.head {
            Some(_) => {
                let mut tail_contents = self.tail
                    .as_deref()
                    .expect("`self.tail` should be Some(_)")
                    .borrow_mut();

                tail_contents.next.replace(Rc::clone(&new_link));
            },
            None => { self.head.replace(Rc::clone(&new_link)); },
        }
        self.tail = Some(new_link);
        self.size += 1;
    }

    /// Creates a consuming iterator, that is, one that moves each value out of
    /// the list (from start to end). The list cannot be used after calling
    /// this.
    ///
    /// # Examples
    /// 
    /// ```
    /// # use ods::singly_linked_list::SLList;
    /// let mut list = SLList::new();
    /// list.push(2);
    /// list.push(1);
    /// list.push(0);
    /// assert_eq!(list.into_iter().collect::<Vec<i32>>(), [0, 1, 2]);
    /// ```
    pub fn into_iter(self) -> IntoIter<T> {
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

/// Iterative implementation of list desctruction.
/// 
/// The reason for this custom implementation is that the default one is 
/// recursive, which has the risk of blowing the stack if the list is large 
/// enough.
impl<T> Drop for SLList<T> {
    fn drop(&mut self) {
        let mut next = self.head.take();
        self.tail.take();
        while let Some(link) = next {
            next = link.borrow_mut().next.take();
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn push_from_empty_returns_values_back() {
        let mut list = SLList { head: None, tail: None, size: 0 };
        list.push('a');
        list.push('b');
        list.push('c');
        assert_eq!(list.into_iter().collect::<Vec<char>>(), ['c', 'b', 'a']);
    }

    #[test]
    fn push_from_empty_returns_correct_size() {
        let mut list = SLList { head: None, tail: None, size: 0 };
        list.push('a');
        assert_eq!(list.size(), 1);
        list.push('b');
        assert_eq!(list.size(), 2);
        list.push('c');
        assert_eq!(list.size(), 3);
    }

    #[test]
    fn pop_from_initialized_returns_contents() {
        let l1 = Rc::new(
            RefCell::new(Node { value: 1, next: None })
        );
        let l1_tail = Rc::clone(&l1);
        let l2 = Rc::new(
            RefCell::new(Node { value: 2, next: Some(l1) })
        );
        let l3 = Rc::new(
            RefCell::new(Node { value: 3, next: Some(l2) })
        );
        let mut list = SLList {
            head: Some(l3), tail: Some(l1_tail), size: 3,
        };
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn pop_from_initialized_keeps_track_of_size() {
        let l1 = Rc::new(
            RefCell::new(Node { value: 1, next: None })
        );
        let l1_tail = Rc::clone(&l1);
        let l2 = Rc::new(
            RefCell::new(Node { value: 2, next: Some(l1) })
        );
        let l3 = Rc::new(
            RefCell::new(Node { value: 3, next: Some(l2) })
        );
        let mut list = SLList {
            head: Some(l3), tail: Some(l1_tail), size: 3,
        };
        assert_eq!(list.size(), 3);
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
    fn add_from_empty_returns_values_back() {
        let mut list = SLList { head: None, tail: None, size: 0 };
        list.add('a');
        list.add('b');
        list.add('c');
        assert_eq!(list.into_iter().collect::<Vec<char>>(), ['a', 'b', 'c']);
    }

    #[test]
    fn add_from_empty_returns_correct_size() {
        let mut list = SLList { head: None, tail: None, size: 0 };
        list.add('a');
        assert_eq!(list.size(), 1);
        list.add('b');
        assert_eq!(list.size(), 2);
        list.add('c');
        assert_eq!(list.size(), 3);
    }
}
