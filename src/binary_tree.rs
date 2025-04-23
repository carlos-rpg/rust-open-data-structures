use std::rc::{Rc, Weak};
use std::cell::RefCell;

type Link<T> = Rc<RefCell<Node<T>>>;
type WeakLink<T> = Weak<RefCell<Node<T>>>;


#[derive(Debug)]
pub struct Node<T> {
    value: T,
    parent: Option<WeakLink<T>>,
    left: Option<Link<T>>,
    right: Option<Link<T>>,
}

impl<T> Node<T> {
    pub fn new(value: T) -> Link<T> {
        Rc::new(RefCell::new(
            Self { value, parent: None, left: None, right: None }
        ))
    }

    pub fn depth(link: &Link<T>) -> usize {
        let mut depth = 0;
        let mut link_opt = link.borrow().parent.clone();

        while let Some(weak_link) = link_opt {
            depth += 1;
            let link = weak_link.upgrade().unwrap();
            link_opt = link.borrow().parent.clone();
        }
        depth
    }

    pub fn size(link: &Link<T>) -> usize {
        let mut size = 0;
        let mut links = vec![Rc::clone(link)];
        let mut link;

        while links.len() > 0 {
            size += 1;
            link = links.remove(0);

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
        unimplemented!()
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    fn build_test_tree() -> HashMap<String, Link<i32>> {
        let root = Node::new(0);
        let l = Node::new(1);
        let r = Node::new(2);
        let rl = Node::new(3);
        let rll = Node::new(4);
        let rlr = Node::new(5);

        root.borrow_mut().left.replace(Rc::clone(&l));
        root.borrow_mut().right.replace(Rc::clone(&r));

        l.borrow_mut().parent.replace(Rc::downgrade(&root));

        r.borrow_mut().parent.replace(Rc::downgrade(&root));
        r.borrow_mut().left.replace(Rc::clone(&rl));

        rl.borrow_mut().parent.replace(Rc::downgrade(&r));
        rl.borrow_mut().left.replace(Rc::clone(&rll));
        rl.borrow_mut().right.replace(Rc::clone(&rlr));

        rll.borrow_mut().parent.replace(Rc::downgrade(&rl));
        rlr.borrow_mut().parent.replace(Rc::downgrade(&rl));

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
        assert_eq!(Node::depth(&links["RLL"]), 3);
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
}
