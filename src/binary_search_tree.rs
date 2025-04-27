use crate::binary_tree::*;

pub struct BinarySearchTree<T> {
    root: Option<Link<T>>,
}

impl<T: PartialOrd> BinarySearchTree<T> {
    pub fn new() -> BinarySearchTree<T> {
        Self { root: None }
    }

    pub fn find(&self, x: T) -> Option<Link<T>> {
        let mut link_opt = self.root.clone();
        while let Some(ref link) = link_opt  {
            let borrow = link.borrow();
            let next_link_opt = if x < borrow.value {
                borrow.left.clone()
            } else if x > borrow.value {
                borrow.right.clone()
            } else {
                break;
            };
            drop(borrow);
            link_opt = next_link_opt;
        }
        link_opt
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::rc::Rc;

    fn build_test_tree() -> BinarySearchTree<i32> {
        let root = Node::new(4);
        let l = Node::new(0);
        let r = Node::new(12);
        let rl = Node::new(7);
        let rll = Node::new(5);
        let rlr = Node::new(9);

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
