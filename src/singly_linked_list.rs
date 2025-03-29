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
        let new_node = Node::new(x, self.head.as_ref());
        self.head = Some(new_node);

        if self.tail.is_none() {
            self.tail = self.head.as_ref().map(|link| Rc::clone(link));
        }
        self.size += 1;
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
