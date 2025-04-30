use crate::binary_tree::*;

pub struct BinarySearchTree<T> {
    root: Option<Link<T>>,
    size: usize,
}

impl<T> BinarySearchTree<T> {
    pub fn new() -> BinarySearchTree<T> {
        Self { root: None, size: 0 }
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        self.root.is_none()
    }
}

impl<T: PartialOrd> BinarySearchTree<T> {
    pub fn find(&self, value: T) -> Option<Link<T>> {
        let value_link = Link::new(value);
        let mut link_opt = self.root.clone();

        while let Some(ref link) = link_opt  {
            link_opt = if value_link < *link {
                link.get_left()
            } else if value_link > *link {
                link.get_right()
            } else {
                break;
            };
        }
        link_opt
    }

    pub fn add(&self, _value: T) -> bool {
        unimplemented!();
    }

    pub fn remove(&self, _value: T) -> bool {
        unimplemented!();
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    fn build_test_tree() -> BinarySearchTree<i32> {
        let root = Link::new(4);
        let l = Link::new(0);
        let r = Link::new(12);
        let rl = Link::new(7);
        let rll = Link::new(5);
        let rlr = Link::new(9);

        root.set_left(&l);
        root.set_right(&r);
        l.set_parent(&root);
        r.set_parent(&root);
        r.set_left(&rl);
        rl.set_parent(&r);
        rl.set_left(&rll);
        rl.set_right(&rlr);
        rll.set_parent(&rl);
        rlr.set_parent(&rl);

        BinarySearchTree { root: Some(root), size: 6 }
    }

    #[test]
    fn find_empty_returns_none() {
        let tree = BinarySearchTree { root: None, size: 0 };
        assert!(tree.find(1).is_none());
    }

    #[test]
    fn find_non_emtpy_returns_some() {
        let tree = build_test_tree();
        assert_eq!(tree.find(4).unwrap(), Link::new(4));
        assert_eq!(tree.find(0).unwrap(), Link::new(0));
        assert_eq!(tree.find(12).unwrap(), Link::new(12));
        assert_eq!(tree.find(7).unwrap(), Link::new(7));
        assert_eq!(tree.find(5).unwrap(), Link::new(5));
        assert_eq!(tree.find(9).unwrap(), Link::new(9));
    }

    #[test]
    fn find_non_empty_returns_none() {
        let tree = build_test_tree();
        assert!(tree.find(-1).is_none());
        assert!(tree.find(101).is_none());
        assert!(tree.find(1).is_none());
    }
}
