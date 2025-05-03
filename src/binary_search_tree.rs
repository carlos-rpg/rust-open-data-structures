use crate::binary_tree::*;

pub struct BinarySearchTree<T> {
    root: Option<RefNode<T>>,
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
    pub fn find(&self, value: T) -> Option<RefNode<T>> {
        let value_node = RefNode::new(value);
        let mut node_opt = self.root.clone();

        while let Some(ref node) = node_opt  {
            node_opt = if value_node < *node {
                node.get_left()
            } else if value_node > *node {
                node.get_right()
            } else {
                break;
            };
        }
        node_opt
    }

    fn find_last(&self, value_node: &RefNode<T>) -> Option<RefNode<T>> {
        let mut node_opt = self.root.clone();
        let mut last_node = None;

        while let Some(node) = node_opt  {
            last_node = Some(RefNode::clone(&node));

            node_opt = if *value_node < node {
                node.get_left()
            } else if *value_node > node {
                node.get_right()
            } else {
                break;
            };
        }
        last_node
    }

    pub fn add(&mut self, value: T) -> bool {
        let new_node = RefNode::new(value);

        match self.find_last(&new_node) {
            None => {
                self.root = Some(new_node);
            },
            Some(last_node) => {
                if new_node < last_node {
                    last_node.set_left(Some(&new_node));
                } else if new_node > last_node {
                    last_node.set_right(Some(&new_node));
                } else {
                    return false;
                }
                new_node.set_parent(&last_node);
            },
        };
        self.size += 1;
        true
    }

    fn remove_partially_branched(&mut self, node: RefNode<T>) {
        let mut child_opt = node.get_left();
        if child_opt.is_none() {
            child_opt = node.get_right();
        }
        let parent_opt = if node.is_root() {
            self.root = child_opt;
            None
        } else {
            let parent = node
                .get_parent()
                .expect("`node` should not be a root one");

            if let Some(left) = parent.get_left() {
                if left == node {
                    parent.set_left(child_opt.as_ref());
                }
            } else {
                parent.set_right(child_opt.as_ref());
            }
            Some(parent)
        };
        if let Some(parent) = parent_opt {
            parent.set_parent(&node);
        }
    }

    fn remove_fully_branched(&self, node: RefNode<T>) {
        let mut min_node = node
            .get_right()
            .expect("`node` should have both children");

        let mut iterated = false;
        while let Some(left) = min_node.get_left() {
            min_node = left;
            iterated = true;
        }
        let last = min_node
            .get_parent()
            .expect("`min_node` should have a parent");

        if iterated {
            last.set_left(None);
        } else {
            last.set_right(None);
        }
        let min_value = min_node
            .into_inner_value()
            .expect("`min_node` should have only 1 reference");

        node.set(min_value);
    }

    pub fn remove(&mut self, value: T) -> bool {
        let node = match self.find(value) {
            None => return false,
            Some(node) => node,
        };
        if node.is_fully_branched() {
            self.remove_fully_branched(node);
        } else {
            self.remove_partially_branched(node);
        }
        self.size -= 1;
        true
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    fn build_test_tree() -> BinarySearchTree<i32> {
        let root = RefNode::new(4);
        let l = RefNode::new(0);
        let r = RefNode::new(12);
        let rl = RefNode::new(7);
        let rll = RefNode::new(5);
        let rlr = RefNode::new(9);

        root.set_left(Some(&l));
        root.set_right(Some(&r));
        l.set_parent(&root);
        r.set_parent(&root);
        r.set_left(Some(&rl));
        rl.set_parent(&r);
        rl.set_left(Some(&rll));
        rl.set_right(Some(&rlr));
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
        assert_eq!(tree.find(4).unwrap(), RefNode::new(4));
        assert_eq!(tree.find(0).unwrap(), RefNode::new(0));
        assert_eq!(tree.find(12).unwrap(), RefNode::new(12));
        assert_eq!(tree.find(7).unwrap(), RefNode::new(7));
        assert_eq!(tree.find(5).unwrap(), RefNode::new(5));
        assert_eq!(tree.find(9).unwrap(), RefNode::new(9));
    }

    #[test]
    fn find_non_empty_returns_none() {
        let tree = build_test_tree();
        assert!(tree.find(-1).is_none());
        assert!(tree.find(101).is_none());
        assert!(tree.find(1).is_none());
    }

    #[test]
    fn add_returns_insertion_outcome() {
        let mut tree = BinarySearchTree { root: None, size : 0 };
        assert!(tree.add(0));
        assert!(tree.add(2));
        assert!(!tree.add(0));
        assert!(tree.add(-2));
        assert!(!tree.add(2));
    }

    #[test]
    fn add_keeps_size_count() {
        let mut tree = BinarySearchTree { root: None, size: 0 };
        assert_eq!(tree.size(), 0);
        tree.add(0);
        assert_eq!(tree.size(), 1);
        tree.add(2);
        assert_eq!(tree.size(), 2);
        tree.add(2);
        assert_eq!(tree.size(), 2);
        tree.add(-2);
        assert_eq!(tree.size(), 3);
    }

    #[test]
    fn add_emtpy_inserts_root() {
        let mut tree = BinarySearchTree { root: None, size: 0 };
        tree.add(0);
        let root = tree.root.clone().unwrap();
        assert_eq!(root, RefNode::new(0));
        assert!(root.get_parent().is_none());
        assert!(root.get_left().is_none());
        assert!(root.get_right().is_none());
    }

    #[test]
    fn add_non_empty_inserts_leaf() {
        let mut tree = build_test_tree();
        tree.add(-1);
        let leaf = tree.root.clone().unwrap().get_left().unwrap().get_left().unwrap();
        assert_eq!(leaf, RefNode::new(-1));
        assert!(leaf.get_left().is_none());
        assert!(leaf.get_right().is_none());
        assert!(leaf.get_parent().is_some());
    }

    #[test]
    fn remove_returns_outcome() {
        let mut tree = build_test_tree();
        assert!(tree.remove(0));
        assert!(!tree.remove(0));
        assert!(tree.remove(12));
        assert!(!tree.remove(12));
        assert!(tree.remove(7));
        assert!(!tree.remove(7));
    }

    #[test]
    fn remove_keeps_track_of_size() {
        let mut tree = build_test_tree();
        assert_eq!(tree.size(), 6);
        tree.remove(0);
        assert_eq!(tree.size(), 5);
        tree.remove(12);
        assert_eq!(tree.size(), 4);
        tree.remove(12);
        assert_eq!(tree.size(), 4);
        tree.remove(7);
        assert_eq!(tree.size(), 3);
    }

    #[test]
    fn remove_takes_value_out_of_tree() {
        let mut tree = build_test_tree();
        tree.remove(0);
        assert!(tree.root.clone().unwrap().get_left().is_none());
        tree.remove(12);
        assert_eq!(tree.root.clone().unwrap().get_right().unwrap(), RefNode::new(7));
        tree.remove(7);
        assert_eq!(tree.root.clone().unwrap().get_right().unwrap(), RefNode::new(9));
        tree.remove(4);
        assert_eq!(tree.root.clone().unwrap(), RefNode::new(9));
    }
}
