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
                    last_node.set_left(&new_node);
                } else if new_node > last_node {
                    last_node.set_right(&new_node);
                } else {
                    return false;
                }
                new_node.set_parent(&last_node);
            },
        };
        self.size += 1;
        true
    }

    fn splice(&mut self, node: RefNode<T>) {
        let mut child_opt = node.get_left();
        if child_opt.is_none() {
            child_opt = node.get_right();
        }
        let parent_opt = if node.get_parent().is_none() {
            self.root = child_opt;
            None
        } else {
            let parent = node
                .get_parent()
                .expect("`node` should not be a root one");

            let child = child_opt
                .as_ref()
                .expect("");

            if let Some(left) = parent.get_left() {
                if left == node {
                    parent.set_left(child);
                }
            } else {
                parent.set_right(child);
            }
            Some(parent)
        };
        if let Some(parent) = parent_opt {
            parent.set_parent(&node);
        }
    }

    pub fn remove(&mut self, value: T) -> bool {
        let node = match self.find(value) {
            None => return false,
            Some(node) => node,
        };
        if node.get_left().is_none() || node.get_right().is_none() {
            self.splice(node);
        } else {
            let mut right_min_node = node
                .get_right()
                .expect("`node` should have both children");

            while let Some(left) = right_min_node.get_left() {
                right_min_node = left;
            }
            let right_min_value = right_min_node
                .into_inner_value()
                .expect("");

            node.set(right_min_value);
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
        let node = RefNode::new(0);
        tree.add(0);
        let root = tree.root.clone().unwrap();
        assert_eq!(root, node);
        assert!(root.get_parent().is_none());
        assert!(root.get_left().is_none());
        assert!(root.get_right().is_none());
    }

    #[test]
    fn add_non_empty_inserts_leaf() {
        let mut tree = build_test_tree();
        tree.add(-1);
        let node = tree.root.clone().unwrap().get_left().unwrap().get_left().unwrap();
        assert_eq!(node, RefNode::new(-1));
        assert!(node.get_left().is_none());
        assert!(node.get_right().is_none());
        assert!(node.get_parent().is_some());
    }
}
