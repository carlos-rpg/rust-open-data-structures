use std::rc::{Rc, Weak};
use std::cell::RefCell;


#[derive(Debug)]
struct Node<T> {
    value: T,
    parent: WeakLink<T>,
    left: Option<Link<T>>,
    right: Option<Link<T>>,
}

impl<T> Node<T> {
    fn new(value: T) -> Node<T> {
        Self { value, parent: WeakLink(Weak::new()), left: None, right: None }
    }
}

impl<T: PartialEq> PartialEq for Node<T> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl<T: PartialOrd> PartialOrd for Node<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.value.partial_cmp(&other.value)
    }
}


#[derive(Debug)]
struct WeakLink<T>(Weak<RefCell<Node<T>>>);

impl<T> WeakLink<T> {
    fn upgrade(&self) -> Option<Link<T>> {
        self.0.upgrade().map(|x| Link(x))
    }
}


#[derive(PartialEq, Debug, PartialOrd)]
pub struct Link<T>(Rc<RefCell<Node<T>>>);

impl<T> Link<T> {
    pub fn new(value: T) -> Self {
        Self(Rc::new(RefCell::new(Node::new(value))))
    }

    pub fn get_parent(&self) -> Option<Link<T>> {
        self.0.borrow().parent.upgrade()
    }

    pub fn set_parent(&self, link: &Link<T>) {
        self.0.borrow_mut().parent = WeakLink(Rc::downgrade(&link.0));
    }

    pub fn get_left(&self) -> Option<Link<T>> {
        self.0.borrow().left.as_ref().map(|x| Link::clone(x))
    }

    pub fn set_left(&self, link: &Link<T>) {
        self.0.borrow_mut().left = Some(Link::clone(link));
    }

    pub fn get_right(&self) -> Option<Link<T>> {
        self.0.borrow().right.as_ref().map(|x| Link::clone(x))
    }

    pub fn set_right(&self, link: &Link<T>) {
        self.0.borrow_mut().right = Some(Link::clone(link));
    }

    pub fn depth(&self) -> usize {
        let mut depth = 0;
        let mut link_opt = self.get_parent();

        while let Some(link) = link_opt {
            depth += 1;
            link_opt = link.get_parent();
        }
        depth
    }

    pub fn size(&self) -> usize {
        let mut size = 0;
        let mut links = vec![Link::clone(&self)];

        while !links.is_empty() {
            size += 1;
            let link = links.remove(0);

            if let Some(left_link) = link.get_left() {
                links.push(left_link);
            }
            if let Some(right_link) = link.get_right() {
                links.push(right_link);
            }
        }
        size
    }

    pub fn height(&self) -> usize {
        fn recurse<T>(link_opt: Option<Link<T>>) -> usize {
            match link_opt {
                None => 0,
                Some(link) => 1 + usize::max(
                    recurse(link.get_left()), 
                    recurse(link.get_right()),
                )
            }
        }
        recurse(Some(self).cloned())
    }
}

impl<T> Clone for Link<T> {
    fn clone(&self) -> Self {
        Link(Rc::clone(&self.0))
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    fn build_test_node<T>(value: T) -> Link<T> {
        Link(Rc::new(RefCell::new(
            Node { value, parent: WeakLink(Weak::new()), left: None, right: None }
        )))
    }

    fn build_test_tree() -> HashMap<String, Link<char>> {
        let root = build_test_node('a');
        let l = build_test_node('b');
        let r = build_test_node('c');
        let rl = build_test_node('d');
        let rll = build_test_node('e');
        let rlr = build_test_node('f');

        root.0.borrow_mut().left = Some(Link::clone(&l));
        root.0.borrow_mut().right = Some(Link::clone(&r));

        l.0.borrow_mut().parent = WeakLink(Rc::downgrade(&root.0));

        r.0.borrow_mut().parent = WeakLink(Rc::downgrade(&root.0));
        r.0.borrow_mut().left = Some(Link::clone(&rl));

        rl.0.borrow_mut().parent = WeakLink(Rc::downgrade(&r.0));
        rl.0.borrow_mut().left = Some(Link::clone(&rll));
        rl.0.borrow_mut().right = Some(Link::clone(&rlr));

        rll.0.borrow_mut().parent = WeakLink(Rc::downgrade(&rl.0));
        rlr.0.borrow_mut().parent = WeakLink(Rc::downgrade(&rl.0));

        let mut links = HashMap::new();
        links.insert(String::from(""), root);
        links.insert(String::from("R"), r);
        links.insert(String::from("L"), l);
        links.insert(String::from("RL"), rl);
        links.insert(String::from("RLL"), rll);
        links.insert(String::from("RLR"), rlr);

        links
    }

    #[test]
    fn depth_returns() {
        let links = build_test_tree();
        assert_eq!(links[""].depth(), 0);
        assert_eq!(links["L"].depth(), 1);
        assert_eq!(links["R"].depth(), 1);
        assert_eq!(links["RL"].depth(), 2);
        assert_eq!(links["RLL"].depth() ,3);
        assert_eq!(links["RLR"].depth(), 3);
    }

    #[test]
    fn size_returns() {
        let links = build_test_tree();
        assert_eq!(links[""].size(), 6);
        assert_eq!(links["L"].size(), 1);
        assert_eq!(links["R"].size(), 4);
        assert_eq!(links["RL"].size(), 3);
        assert_eq!(links["RLL"].size(), 1);
        assert_eq!(links["RLR"].size(), 1);
    }

    #[test]
    fn height_returns() {
        let links = build_test_tree();
        assert_eq!(links[""].height(), 4);
        assert_eq!(links["L"].height(), 1);
        assert_eq!(links["R"].height(), 3);
        assert_eq!(links["RL"].height(), 2);
        assert_eq!(links["RLL"].height(), 1);
        assert_eq!(links["RLR"].height(), 1);
    }
}
