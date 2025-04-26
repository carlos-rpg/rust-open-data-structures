use std::rc::{Rc, Weak};
use std::cell::RefCell;

pub type Link<T> = Option<Rc<RefCell<Node<T>>>>;
pub type WeakLink<T> = Weak<RefCell<Node<T>>>;


pub struct Node<T> {
    pub value: T,
    pub parent: WeakLink<T>,
    pub left: Link<T>,
    pub right: Link<T>,
}

impl<T> Node<T> {
    pub fn new(value: T) -> Link<T> {
        Some(Rc::new(RefCell::new(
            Self { value, parent: Weak::new(), left: None, right: None }
        )))
    }

    pub fn depth(link: &Link<T>) -> Option<usize> {
        let mut depth = 0;
        let mut link_opt = link.as_ref()?.borrow().parent.upgrade();

        while let Some(link) = link_opt {
            depth += 1;
            link_opt = link.borrow().parent.upgrade();
        }
        Some(depth)
    }

    pub fn size(link: &Link<T>) -> usize {
        let mut size = 0;
        let mut links = match link {
            None => vec![],
            Some(x) => vec![Rc::clone(x)],
        };
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

    pub fn height(link: &Link<T>) -> usize {
        match link {
            None => 0,
            Some(next) => 1 + usize::max(
                Self::height(&next.borrow().left), 
                Self::height(&next.borrow().right),
            )
        }
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
        links.insert(String::from(""), Some(root));
        links.insert(String::from("R"), Some(r));
        links.insert(String::from("L"), Some(l));
        links.insert(String::from("RL"), Some(rl));
        links.insert(String::from("RLL"), Some(rll));
        links.insert(String::from("RLR"), Some(rlr));

        links
    }

    #[test]
    fn depth_some_link_returns() {
        let links = build_test_tree();
        assert_eq!(Node::depth(&links[""]), Some(0));
        assert_eq!(Node::depth(&links["R"]), Some(1));
        assert_eq!(Node::depth(&links["L"]), Some(1));
        assert_eq!(Node::depth(&links["RL"]), Some(2));
        assert_eq!(Node::depth(&links["RLL"]), Some(3));
        assert_eq!(Node::depth(&links["RLR"]), Some(3));
    }

    #[test]
    fn depth_none_link_returns_zero() {
        assert_eq!(Node::<i32>::depth(&None), None);
    }

    #[test]
    fn size_some_link_returns_non_zero() {
        let links = build_test_tree();
        assert_eq!(Node::size(&links[""]), 6);
        assert_eq!(Node::size(&links["L"]), 1);
        assert_eq!(Node::size(&links["R"]), 4);
        assert_eq!(Node::size(&links["RL"]), 3);
        assert_eq!(Node::size(&links["RLL"]), 1);
        assert_eq!(Node::size(&links["RLR"]), 1);
    }

    #[test]
    fn size_none_link_returns_zero() {
        assert_eq!(Node::<i32>::size(&None), 0);
    }

    #[test]
    fn height_some_link_returns_non_zero() {
        let links = build_test_tree();
        assert_eq!(Node::height(&links[""]), 4);
        assert_eq!(Node::height(&links["L"]), 1);
        assert_eq!(Node::height(&links["R"]), 3);
        assert_eq!(Node::height(&links["RL"]), 2);
        assert_eq!(Node::height(&links["RLL"]), 1);
        assert_eq!(Node::height(&links["RLR"]), 1);
    }

    #[test]
    fn height_none_link_returns_zero() {
        assert_eq!(Node::<i32>::height(&None), 0);
    }
}
