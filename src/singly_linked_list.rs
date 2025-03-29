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
    fn new(x: T) -> Self {
        Self { value: x, next: None }
    }
}

impl<T> SLList<T> {
    pub fn new() -> Self {
        Self { head: None, tail: None, size: 0 }
    }

    pub fn size(&self) -> usize {
        self.size
    }
}
