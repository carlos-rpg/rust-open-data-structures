use std::rc::Rc;

type Link<T> = Rc<Node<T>>;

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
    fn new(value: T, next: Option<&Link<T>>) -> Rc<Self> {
        Rc::new(
            Self { value, next: next.map(|link| Rc::clone(link)) }
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
        self.head = Some(Node::new(x, self.head.as_ref()));

        if self.tail.is_none() {
            self.tail = self.head.as_ref().map(|link| Rc::clone(link));
        }
        self.size += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        let pop_link = Rc::clone(self.head.as_ref()?);
        self.head = pop_link.next.as_ref().map(|link| Rc::clone(link));
        self.size -= 1;

        if self.head.is_none() {
            self.tail = None;
        }
        Rc::into_inner(pop_link).map(|node| node.value)
    }

    pub fn iter(&self) -> SLListIter<T> {
        SLListIter { ref_to: &self.head }
    }
}

pub struct SLListIter<'a, T> {
    ref_to: &'a Option<Link<T>>,
}

impl<'a, T> Iterator for SLListIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let item = self.ref_to.as_ref()?;
        self.ref_to = &item.next;
        Some(&item.value)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn iter_empty_only_returns_none() {
        let list: SLList<i32> = SLList { head: None, tail: None, size: 0 };
        let mut list_iter = list.iter();
        assert_eq!(list_iter.next(), None);
        assert_eq!(list_iter.next(), None);
        assert_eq!(list_iter.next(), None);
    }

    #[test]
    fn iter_one_value_returns_one_value() {
        let mut list = SLList {
            head: Some(Rc::new(Node { value: 0, next: None })),
            tail: None,
            size: 1,
        };
        list.tail = Some(Rc::clone(list.head.as_ref().unwrap()));
        let mut list_iter = list.iter();
        assert_eq!(list_iter.next(), Some(&0));
        assert_eq!(list_iter.next(), None);
        assert_eq!(list_iter.next(), None);
    }

    #[test]
    fn iter_many_values_returns_two_values() {
        let n1 = Rc::new(
            Node { value: 1, next: None }
        );
        let n2 = Rc::new(
            Node { value: 2, next: Some(Rc::clone(&n1)) }
        );
        let list = SLList { head: Some(n2), tail: Some(n1), size: 2 };
        let mut list_iter = list.iter();
        assert_eq!(list_iter.next(), Some(&2));
        assert_eq!(list_iter.next(), Some(&1));
        assert_eq!(list_iter.next(), None);
    }

    #[test]
    fn push_from_empty_returns_values_back() {
        let mut list = SLList { head: None, tail: None, size: 0 };
        list.push('a');
        list.push('b');
        list.push('c');
        assert_eq!(list.iter().collect::<Vec<&char>>(), [&'c', &'b', &'a']);
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
        let n1 = Rc::new(
            Node { value: 1, next: None }
        );
        let n1_tail = Rc::clone(&n1);
        let n2 = Rc::new(
            Node { value: 2, next: Some(n1) }
        );
        let n3 = Rc::new(
            Node { value: 3, next: Some(n2) }
        );
        let mut list = SLList {
            head: Some(n3), tail: Some(n1_tail), size: 3,
        };
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn pop_from_initialized_keeps_track_of_size() {
        let n1 = Rc::new(
            Node { value: 1, next: None }
        );
        let n1_tail = Rc::clone(&n1);
        let n2 = Rc::new(
            Node { value: 2, next: Some(n1) }
        );
        let n3 = Rc::new(
            Node { value: 3, next: Some(n2) }
        );
        let mut list = SLList {
            head: Some(n3), tail: Some(n1_tail), size: 3,
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
}
