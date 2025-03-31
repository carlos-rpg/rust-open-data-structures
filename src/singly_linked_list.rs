use std::rc::Rc;
use std::cell::RefCell;

type Link<T> = Rc<RefCell<Node<T>>>;

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
        Rc::new(
            RefCell::new(
                Self { value, next: next.map(|link| Rc::clone(link)) }
            )
        )
    }
}

impl<T> SLList<T> {
    pub fn new() -> Self {
        Self { head: None, tail: None, size: 0 }
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn push(&mut self, x: T) {
        let new_link = Node::new(x, self.head.as_ref());
        self.head = Some(Rc::clone(&new_link));

        if self.tail.is_none() {
            self.tail = Some(new_link);
        }
        self.size += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        let pop_link = Rc::clone(self.head.as_ref()?);
        self.head = pop_link.borrow_mut().next.take();

        if self.head.is_none() {
            self.tail = None;
        }
        let pop_contents = Rc::into_inner(pop_link)
            .expect("Rc strong count is not 1")
            .into_inner();

        self.size -= 1;
        Some(pop_contents.value)
    }

    pub fn add(&mut self, x: T) {
        let new_link = Node::new(x, None);

        match self.head {
            Some(_) => {
                let mut tail_contents = self.tail
                    .as_deref()
                    .expect("Tail is `None` but head is `Some(_)`")
                    .borrow_mut();

                tail_contents.next.replace(Rc::clone(&new_link));
            },
            None => { self.head.replace(Rc::clone(&new_link)); },
        }
        self.tail = Some(new_link);
        self.size += 1;
    }

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
