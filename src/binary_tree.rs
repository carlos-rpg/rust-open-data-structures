use std::rc::{Rc, Weak};
use std::cell::RefCell;


pub struct Link<T>(Rc<RefCell<Node<T>>>);

pub struct Node<T> {
    value: T,
    parent: Weak<RefCell<Node<T>>>,
    left: Option<Rc<RefCell<Node<T>>>>,
    right: Option<Rc<RefCell<Node<T>>>>,
}

impl<T> Node<T> {
    pub fn new(value: T) -> Node<T> {
        Self { value, parent: Weak::new(), left: None, right: None }
    }
}


impl<T> Link<T> {
    pub fn new(value: T) -> Self {
        Self(Rc::new(RefCell::new(Node::new(value))))
    }

    pub fn depth(&self) -> usize {
        let mut depth = 0;
        let mut link_opt = self.0.borrow().parent.upgrade();

        while let Some(parent_link) = link_opt {
            depth += 1;
            link_opt = parent_link.borrow().parent.upgrade();
        }
        depth
    }

    pub fn size(&self) -> usize {
        let mut size = 0;
        let mut links = vec![Rc::clone(&self.0)];
        while !links.is_empty() {
            size += 1;
            let link = links.remove(0);
            if let Some(left_link) = &link.borrow().left {
                links.push(Rc::clone(left_link));
            }
            if let Some(right_link) = &link.borrow().right {
                links.push(Rc::clone(right_link));
            }
        }
        size
    }

    pub fn height(&self) -> usize {
        fn recurse<T>(link_opt: &Option<Rc<RefCell<Node<T>>>>) -> usize {
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


#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    fn build_test_node<T>(value: T) -> Rc<RefCell<Node<T>>> {
        Rc::new(RefCell::new(
            Node { value, parent: Weak::new(), left: None, right: None }
        ))
    }

    fn build_test_tree() -> HashMap<String, Link<i32>> {
        let root = build_test_node(0);
        let l = build_test_node(1);
        let r = build_test_node(2);
        let rl = build_test_node(3);
        let rll = build_test_node(4);
        let rlr = build_test_node(5);

        root.borrow_mut().left.replace(Rc::clone(&l));
        root.borrow_mut().right.replace(Rc::clone(&r));

        l.borrow_mut().parent = Rc::downgrade(&root);

        r.borrow_mut().parent = Rc::downgrade(&root);
        r.borrow_mut().left.replace(Rc::clone(&rl));

        rl.borrow_mut().parent = Rc::downgrade(&r);
        rl.borrow_mut().left.replace(Rc::clone(&rll));
        rl.borrow_mut().right.replace(Rc::clone(&rlr));

        rll.borrow_mut().parent = Rc::downgrade(&rl);
        rlr.borrow_mut().parent = Rc::downgrade(&rl);

        let mut links = HashMap::new();
        links.insert(String::from(""), Link(root));
        links.insert(String::from("R"), Link(r));
        links.insert(String::from("L"), Link(l));
        links.insert(String::from("RL"), Link(rl));
        links.insert(String::from("RLL"), Link(rll));
        links.insert(String::from("RLR"), Link(rlr));

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
