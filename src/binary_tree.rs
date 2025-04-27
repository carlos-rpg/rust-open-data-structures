use std::rc::{Rc, Weak};
use std::cell::{Ref, RefCell, RefMut};


pub struct Node<T> {
    pub value: T,
    pub parent: WeakLink<T>,
    pub left: Option<Link<T>>,
    pub right: Option<Link<T>>,
}

impl<T> Node<T> {
    pub fn new(value: T) -> Node<T> {
        Self { value, parent: WeakLink(Weak::new()), left: None, right: None }
    }
}


pub struct WeakLink<T>(Weak<RefCell<Node<T>>>);

impl<T> WeakLink<T> {
    pub fn upgrade(&self) -> Option<Link<T>> {
        self.0.upgrade().map(|x| Link(x))
    }
}


pub struct Link<T>(Rc<RefCell<Node<T>>>);

impl<T> Link<T> {
    pub fn new(value: T) -> Self {
        Self( Rc::new(RefCell::new(Node::new(value))) )
    }

    pub fn borrow(&self) -> Ref<Node<T>> {
        self.0.borrow()
    }

    pub fn borrow_mut(&self) -> RefMut<Node<T>> {
        self.0.borrow_mut()
    }

    pub fn downgrade(&self) -> WeakLink<T> {
        WeakLink(Rc::downgrade(&self.0))
    }

    pub fn depth(&self) -> usize {
        let mut depth = 0;
        let mut link_opt = self.borrow().parent.upgrade();

        while let Some(link) = link_opt {
            depth += 1;
            link_opt = link.borrow().parent.upgrade();
        }
        depth
    }

    pub fn size(&self) -> usize {
        let mut size = 0;
        let mut links = vec![Link::clone(&self)];
        while !links.is_empty() {
            size += 1;
            let link = links.remove(0);
            if let Some(left_link) = &link.borrow().left {
                links.push(Link::clone(left_link));
            }
            if let Some(right_link) = &link.borrow().right {
                links.push(Link::clone(right_link));
            }
        }
        size
    }

    pub fn height(&self) -> usize {
        fn recurse<T>(link_opt: &Option<Link<T>>) -> usize {
            match link_opt {
                None => 0,
                Some(link) => 1 + usize::max(
                    recurse(&link.borrow().left), 
                    recurse(&link.borrow().right),
                )
            }
        }
        1 + usize::max(
            recurse(&self.0.borrow().left), 
            recurse(&self.0.borrow().right),
        )
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

    fn build_test_tree() -> HashMap<String, Link<i32>> {
        let root = build_test_node(0);
        let l = build_test_node(1);
        let r = build_test_node(2);
        let rl = build_test_node(3);
        let rll = build_test_node(4);
        let rlr = build_test_node(5);

        root.borrow_mut().left.replace(Link::clone(&l));
        root.borrow_mut().right.replace(Link::clone(&r));

        l.borrow_mut().parent = Link::downgrade(&root);

        r.borrow_mut().parent = Link::downgrade(&root);
        r.borrow_mut().left.replace(Link::clone(&rl));

        rl.borrow_mut().parent = Link::downgrade(&r);
        rl.borrow_mut().left.replace(Link::clone(&rll));
        rl.borrow_mut().right.replace(Link::clone(&rlr));

        rll.borrow_mut().parent = Link::downgrade(&rl);
        rlr.borrow_mut().parent = Link::downgrade(&rl);

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
