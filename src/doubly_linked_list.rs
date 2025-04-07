//! A safe, doubly linked list

use std::cell::{RefCell, Ref};
use std::rc::Rc;

type Link<T> = Rc<RefCell<Node<T>>>;


pub struct DLList<T> {
    head: Option<Link<T>>,
    tail: Option<Link<T>>,
    size: usize,
}

struct Node<T> {
    value: T,
    next: Option<Link<T>>,
    prev: Option<Link<T>>,
}

impl<T> Node<T> {
    fn new(value: T, next: Option<&Link<T>>, prev: Option<&Link<T>>) -> Link<T> {
        Rc::new(RefCell::new(
            Self {
                value,
                next: next.map(|link| Rc::clone(link)),
                prev: prev.map(|link| Rc::clone(link)),
            }
        ))
    }
}

impl<T> DLList<T> {
    pub fn new() -> Self {
        Self { head: None, tail: None, size: 0 }
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn push_head(&mut self, x: T) {
        let new_head = Node::new(x, self.head.as_ref(), None);
        
        match self.tail.as_ref() {
            None => self.tail = Some(Rc::clone(&new_head)),
            Some(_) => {
                let mut head_node = self.head
                    .as_ref()
                    .expect("`self.head` should be `Some(_)`")
                    .borrow_mut();

                head_node.prev = Some(Rc::clone(&new_head));
            }
        }
        self.head = Some(new_head);
        self.size += 1;
    }

    pub fn push_tail(&mut self, x: T) {
        let new_tail = Node::new(x, None, self.tail.as_ref());

        match self.head.as_ref() {
            None => self.head = Some(Rc::clone(&new_tail)),
            Some(_) => {
                let mut tail_node = self.tail
                    .as_ref()
                    .expect("`self.tail` should be `Some(_)`")
                    .borrow_mut();

                tail_node.next = Some(Rc::clone(&new_tail));
            }
        }
        self.tail = Some(new_tail);
        self.size += 1;
    }

    pub fn pop_head(&mut self) -> Option<T> {
        let old_head = Rc::clone(self.head.as_ref()?);
        self.head = old_head.borrow_mut().next.take();
        
        match self.head.as_ref() {
            None => self.tail = None,
            Some(link) => link.borrow_mut().prev = None,
        }
        let old_node = Rc::into_inner(old_head)
            .expect("`old_head` should have 1 strong reference")
            .into_inner();
        
        self.size -= 1;
        Some(old_node.value)
    }

    pub fn pop_tail(&mut self) -> Option<T> {
        let old_tail = Rc::clone(self.tail.as_ref()?);
        self.tail = old_tail.borrow_mut().prev.take();

        match self.tail.as_ref() {
            None => self.head = None,
            Some(link) => link.borrow_mut().next = None,
        }
        let old_node = Rc::into_inner(old_tail)
            .expect("`old_tail` should have 1 strong reference")
            .into_inner();
        
        self.size -= 1;
        Some(old_node.value)
    }

    pub fn get_head(&self) -> Option<Ref<T>> {
        let ref_node = self.head.as_ref()?.borrow();
        Some(Ref::map(ref_node, |node| &node.value))
    }

    pub fn get_tail(&self) -> Option<Ref<T>> {
        let ref_node = self.tail.as_ref()?.borrow();
        Some(Ref::map(ref_node, |node| &node.value))
    }
}

pub struct IntoIter<T>(DLList<T>);

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop_head()
    }
}

impl<T> DoubleEndedIterator for IntoIter<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0.pop_tail()
    }
}

impl<T> IntoIterator for DLList<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;
    fn into_iter(self) -> Self::IntoIter {
        IntoIter(self)
    }
}

impl<T> Drop for DLList<T> {
    fn drop(&mut self) {
        let mut head_item = self.head.take();
        let mut tail_item = self.tail.take();
        while let (Some(head_node), Some(tail_node)) = (head_item, tail_item) {
            head_item = head_node.borrow_mut().next.take();
            tail_item = tail_node.borrow_mut().prev.take();
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pop_head_returns_contents() {
        let l1 = Rc::new(
            RefCell::new(Node { value: 1, next: None, prev: None})
        );
        let l2 = Rc::new(
            RefCell::new(Node { value: 2, next: Some(Rc::clone(&l1)), prev: None })
        );
        let l3 = Rc::new(
            RefCell::new(Node { value: 3, next: Some(Rc::clone(&l2)), prev: None })
        );
        l2.borrow_mut().prev = Some(Rc::clone(&l3));
        l1.borrow_mut().prev = Some(Rc::clone(&l2));
        drop(l2);

        let mut list = DLList {
            head: Some(l3), tail: Some(l1), size: 3,
        };
        assert_eq!(list.pop_head(), Some(3));
        assert_eq!(list.pop_head(), Some(2));
        assert_eq!(list.pop_head(), Some(1));
        assert_eq!(list.pop_head(), None);
        assert_eq!(list.pop_head(), None);
    }

    #[test]
    fn pop_head_keeps_track_of_size() {
        let l1 = Rc::new(
            RefCell::new(Node { value: 1, next: None, prev: None})
        );
        let l2 = Rc::new(
            RefCell::new(Node { value: 2, next: Some(Rc::clone(&l1)), prev: None })
        );
        let l3 = Rc::new(
            RefCell::new(Node { value: 3, next: Some(Rc::clone(&l2)), prev: None })
        );
        l2.borrow_mut().prev = Some(Rc::clone(&l3));
        l1.borrow_mut().prev = Some(Rc::clone(&l2));
        drop(l2);

        let mut list = DLList {
            head: Some(l3), tail: Some(l1), size: 3,
        };
        assert_eq!(list.size(), 3);
        list.pop_head();
        assert_eq!(list.size(), 2);
        list.pop_head();
        assert_eq!(list.size(), 1);
        list.pop_head();
        assert_eq!(list.size(), 0);
        list.pop_head();
        assert_eq!(list.size(), 0);
    }

    #[test]
    fn pop_tail_returns_contents() {
        let l1 = Rc::new(
            RefCell::new(Node { value: 1, next: None, prev: None})
        );
        let l2 = Rc::new(
            RefCell::new(Node { value: 2, next: Some(Rc::clone(&l1)), prev: None })
        );
        let l3 = Rc::new(
            RefCell::new(Node { value: 3, next: Some(Rc::clone(&l2)), prev: None })
        );
        l2.borrow_mut().prev = Some(Rc::clone(&l3));
        l1.borrow_mut().prev = Some(Rc::clone(&l2));
        drop(l2);

        let mut list = DLList {
            head: Some(l3), tail: Some(l1), size: 3,
        };
        assert_eq!(list.pop_tail(), Some(1));
        assert_eq!(list.pop_tail(), Some(2));
        assert_eq!(list.pop_tail(), Some(3));
        assert_eq!(list.pop_tail(), None);
        assert_eq!(list.pop_head(), None);
    }

    #[test]
    fn pop_tail_keeps_track_of_size() {
        let l1 = Rc::new(
            RefCell::new(Node { value: 1, next: None, prev: None})
        );
        let l2 = Rc::new(
            RefCell::new(Node { value: 2, next: Some(Rc::clone(&l1)), prev: None })
        );
        let l3 = Rc::new(
            RefCell::new(Node { value: 3, next: Some(Rc::clone(&l2)), prev: None })
        );
        l2.borrow_mut().prev = Some(Rc::clone(&l3));
        l1.borrow_mut().prev = Some(Rc::clone(&l2));
        drop(l2);

        let mut list = DLList {
            head: Some(l3), tail: Some(l1), size: 3,
        };
        assert_eq!(list.size(), 3);
        list.pop_tail();
        assert_eq!(list.size(), 2);
        list.pop_tail();
        assert_eq!(list.size(), 1);
        list.pop_tail();
        assert_eq!(list.size(), 0);
        list.pop_tail();
        assert_eq!(list.size(), 0);
    }

    #[test]
    fn push_head_returns_contents() {
        let mut list = DLList { head: None, tail: None, size: 0 };
        list.push_head('a');
        list.push_head('b');
        list.push_head('c');
        assert_eq!(list.into_iter().collect::<Vec<char>>(), ['c', 'b', 'a']);
    }

    #[test]
    fn push_head_keeps_track_of_size() {
        let mut list = DLList { head: None, tail: None, size: 0 };
        list.push_head('a');
        assert_eq!(list.size(), 1);
        list.push_head('b');
        assert_eq!(list.size(), 2);
        list.push_head('c');
        assert_eq!(list.size(), 3);
    }

    #[test]
    fn push_tail_returns_contents() {
        let mut list = DLList { head: None, tail: None, size: 0 };
        list.push_tail('a');
        list.push_tail('b');
        list.push_tail('c');
        assert_eq!(list.into_iter().collect::<Vec<char>>(), ['a', 'b', 'c']);
    }

    #[test]
    fn push_tail_keeps_track_of_size() {
        let mut list = DLList { head: None, tail: None, size: 0 };
        list.push_tail('a');
        assert_eq!(list.size(), 1);
        list.push_tail('b');
        assert_eq!(list.size(), 2);
        list.push_tail('c');
        assert_eq!(list.size(), 3);
    }
}