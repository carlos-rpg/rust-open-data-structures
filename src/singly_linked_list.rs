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
    /// let list: SLList<i32> = SLList::initialize();
    /// ```
    pub fn initialize() -> Self {
        Self { head: None, size: 0 }
    }

    /// Returns the number of element in the list.
    /// 
    /// # Examples
    /// 
    /// ```
    /// # use ods::singly_linked_list::SLList;
    /// let list: SLList<i32> = SLList::initialize();
    /// assert_eq!(list.size(), 0);
    /// ```
    pub fn size(&self) -> usize {
        self.size
    }

    /// Inserts a value as the new head of the list.
    /// 
    /// # Examples
    /// 
    /// ```
    /// # use ods::singly_linked_list::SLList;
    /// let mut list = SLList::initialize();
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
    /// let mut list = SLList::initialize();
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
    /// let mut list = SLList::initialize();
    /// list.push('a');
    /// assert_eq!(list.iter().next(), Some(&'a'));
    /// ```
    pub fn iter(&self) -> Iter<'_, T> {
        Iter(self.head.as_ref())
    }

    /// Returns an iterator of mutable references to the list's values.
    /// 
    /// # Examples
    /// 
    /// ```
    /// # use ods::singly_linked_list::SLList;
    /// let mut list = SLList::initialize();
    /// list.push('a');
    /// assert_eq!(list.iter_mut().next(), Some(&mut 'a'));
    /// ```
    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut(self.head.as_mut())
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

    #[test]
    fn initialize_has_size_zero() {
        let list = SLList::<i32>::initialize();
        assert_eq!(list.size(), 0);
    }

    #[test]
    fn initialize_returns_empty_list() {
        let list = SLList::<i32>::initialize();
        assert_eq!(list.iter().count(), 0);
    }

    #[test]
    fn push_updates_storage() {
        let mut list = SLList::initialize();
        list.push('a');
        assert_eq!(list.iter().collect::<Vec<&char>>(), [&'a']);
        list.push('b');
        assert_eq!(list.iter().collect::<Vec<&char>>(), [&'b', &'a']);
        list.push('c');
        assert_eq!(list.iter().collect::<Vec<&char>>(), [&'c', &'b', &'a']);
    }

    #[test]
    fn push_updates_size() {
        let mut list = SLList::initialize();
        list.push('a');
        assert_eq!(list.size(), 1);
        list.push('b');
        assert_eq!(list.size(), 2);
        list.push('c');
        assert_eq!(list.size(), 3);
    }

    #[test]
    fn pop_updates_storage() {
        let mut list = SLList::initialize();
        list.push('a');
        list.push('b');
        list.push('c');

        list.pop();
        assert_eq!(list.iter().collect::<Vec<&char>>(), [&'b', &'a']);
        list.pop();
        assert_eq!(list.iter().collect::<Vec<&char>>(), [&'a']);
        list.pop();
        assert!(list.iter().collect::<Vec<&char>>().is_empty());
        list.pop();
        assert!(list.iter().collect::<Vec<&char>>().is_empty());
    }

    #[test]
    fn pop_updates_size() {
        let mut list = SLList::initialize();
        list.push('a');
        list.push('b');
        list.push('c');

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
    fn pop_returns_head() {
        let mut list = SLList::initialize();
        list.push('a');
        list.push('b');
        list.push('c');

        assert_eq!(list.pop(), Some('c'));
        assert_eq!(list.pop(), Some('b'));
        assert_eq!(list.pop(), Some('a'));
        assert_eq!(list.pop(), None);
        assert_eq!(list.pop(), None); 
    }
}
