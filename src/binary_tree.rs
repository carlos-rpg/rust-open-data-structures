use std::rc::{Rc, Weak};
use std::cell::RefCell;

pub type Link<T> = Rc<RefCell<Node<T>>>;
pub type WeakLink<T> = Weak<RefCell<Node<T>>>;


pub struct Node<T> {
    pub value: T,
    pub parent: WeakLink<T>,
    pub left: Option<Link<T>>,
    pub right: Option<Link<T>>,
}

impl<T> Node<T> {
    pub fn new(value: T) -> Link<T> {
        Rc::new(RefCell::new(
            Self { value, parent: Weak::new(), left: None, right: None }
        ))
    }

    pub fn depth(link: &Link<T>) -> usize {
        let mut depth = 0;
        let mut link_opt = link.borrow().parent.upgrade();

        while let Some(parent_link) = link_opt {
            depth += 1;
            link_opt = parent_link.borrow().parent.upgrade();
        }
        depth
    }

    pub fn size(link: &Link<T>) -> usize {
        let mut size = 0;
        let mut links = vec![Rc::clone(link)];
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
            recurse(&link.borrow().left), 
            recurse(&link.borrow().right),
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
        assert_eq!(Node::depth(&links[""]), 0);
        assert_eq!(Node::depth(&links["R"]), 1);
        assert_eq!(Node::depth(&links["L"]), 1);
        assert_eq!(Node::depth(&links["RL"]), 2);
        assert_eq!(Node::depth(&links["RLL"]),3);
        assert_eq!(Node::depth(&links["RLR"]), 3);
    }

    #[test]
    fn size_returns() {
        let links = build_test_tree();
        assert_eq!(Node::size(&links[""]), 6);
        assert_eq!(Node::size(&links["L"]), 1);
        assert_eq!(Node::size(&links["R"]), 4);
        assert_eq!(Node::size(&links["RL"]), 3);
        assert_eq!(Node::size(&links["RLL"]), 1);
        assert_eq!(Node::size(&links["RLR"]), 1);
    }

    #[test]
    fn height_returns() {
        let links = build_test_tree();
        assert_eq!(Node::height(&links[""]), 4);
        assert_eq!(Node::height(&links["L"]), 1);
        assert_eq!(Node::height(&links["R"]), 3);
        assert_eq!(Node::height(&links["RL"]), 2);
        assert_eq!(Node::height(&links["RLL"]), 1);
        assert_eq!(Node::height(&links["RLR"]), 1);
    }
}
