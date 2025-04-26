use crate::binary_tree::*;

pub struct BinarySearchTree<T> {
    root: Link<T>,
}

impl<T: PartialOrd> BinarySearchTree<T> {
    pub fn new() -> BinarySearchTree<T> {
        Self { root: None }
    }

    pub fn find(&self, x: T) -> Link<T> {
        let mut target = self.root.clone();

        while let Some(link) = target.clone()  {
            let borrow = link.borrow();

            let next = if x < borrow.value {
                &borrow.left
            } else if x > borrow.value {
                &borrow.right
            } else {
                break;
            };
            target = next.clone();
        }
        target
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::rc::{Rc, Weak};
    use std::cell::RefCell;

    fn build_test_node<T>(value: T) -> Rc<RefCell<Node<T>>> {
        Rc::new(RefCell::new(
            Node { value, parent: Weak::new(), left: None, right: None }
        ))
    }

    fn build_test_tree() -> BinarySearchTree<i32> {
        let root = build_test_node(4);
        let l = build_test_node(0);
        let r = build_test_node(12);
        let rl = build_test_node(7);
        let rll = build_test_node(5);
        let rlr = build_test_node(9);

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

        BinarySearchTree { root: Some(root) }
    }

    #[test]
    fn find_empty_returns_none() {
        let tree = BinarySearchTree { root: None };
        assert!(tree.find(1).is_none());
    }

    #[test]
    fn find_non_emtpy_returns_some() {
        let tree = build_test_tree();
        let link_4 = tree.find(4).unwrap().borrow().value;
        let link_0 = tree.find(0).unwrap().borrow().value;
        let link_12 = tree.find(12).unwrap().borrow().value;
        let link_7 = tree.find(7).unwrap().borrow().value;
        let link_5 = tree.find(5).unwrap().borrow().value;
        let link_9 = tree.find(9).unwrap().borrow().value;

        assert_eq!(link_4, 4);
        assert_eq!(link_0, 0);
        assert_eq!(link_12, 12);
        assert_eq!(link_7, 7);
        assert_eq!(link_5, 5);
        assert_eq!(link_9, 9);
    }

    #[test]
    fn find_non_empty_returns_none() {
        let tree = build_test_tree();

        assert!(tree.find(-1).is_none());
        assert!(tree.find(101).is_none());
        assert!(tree.find(1).is_none());
    }
}
