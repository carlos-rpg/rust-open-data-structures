use std::rc::{Rc, Weak};
use std::cell::OnceCell;

type Link = Rc<OnceCell<Node>>;
type WeakLink = Weak<OnceCell<Node>>;

#[derive(Debug)]
pub struct BinaryTree {
    root: Link,
}


#[derive(Debug)]
pub struct Node {
    loc: String,
    parent: WeakLink,
    left: Link,
    right: Link,
}

impl Node {
    fn new(loc: String, parent: &Link) -> Node {
        Self { 
            loc,
            parent: Rc::downgrade(parent),
            left: Rc::new(OnceCell::new()),
            right: Rc::new(OnceCell::new()),
        }
    }
}


impl BinaryTree {
    pub fn new() -> Self {
        Self { root: Rc::new(OnceCell::new()) }
    }

    pub fn is_empty(&self) -> bool {
        self.root.get().is_none()
    }

    pub fn push(&self, loc: String) -> &Link {
        let mut link = &self.root;
        for ch in loc.chars() {
            let node = link.get().expect("Branch cell should be initialized");

            link = match ch {
                'L' => &node.left,
                'R' => &node.right,
                _ => panic!("char {} in `loc` must be 'L' or 'R'", ch),
            };
        };
        let node = if loc.is_empty() {
            Node::new(loc, &Rc::new(OnceCell::new()))
        } else {
            Node::new(loc, link)
        };
        link.set(node).expect("Leaf cell should be uninitialized");
        link
    }

    pub fn depth_recursive(link: &Link) -> usize {
        let node = link
            .get()
            .expect("`link` node should always be initialized");

        match node.parent.upgrade() {
            None => 0,
            Some(parent_link) => 1 + Self::depth_recursive(&parent_link),
        }
    }

    pub fn depth_iterative(link: &Link) -> usize {
        let mut depth = 0;
        let mut link_rc = Rc::clone(link);
        let mut node = link_rc
            .get()
            .expect("`link` node should always be initialized");

        while let Some(link_parent) = node.parent.upgrade() {
            depth += 1;
            link_rc = Rc::clone(&link_parent);
            node = link_rc
                .get()
                .expect("`link` node should always be initialized");
        }
        depth
    }

    pub fn size_recursive(link: &Link) -> usize {
        match link.get() {
            None => 0,
            Some(node) => 1 + Self::size_recursive(&node.left) + Self::size_recursive(&node.right),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    fn build_test_tree() -> (BinaryTree, HashMap<String, Rc<OnceCell<Node>>>) {
        let tree = BinaryTree { root: Rc::new(OnceCell::new()) };

        let root = Rc::clone(&tree.root);
        let _ = root.set(
            Node { 
                loc: String::from(""),
                parent: Rc::downgrade(&Rc::new(OnceCell::new())),
                left: Rc::new(OnceCell::new()),
                right: Rc::new(OnceCell::new()),
            }
        );

        let r = Rc::clone(&root.get().unwrap().right);
        let _ = r.set(
            Node { 
                loc: String::from("R"),
                parent: Rc::downgrade(&root),
                left: Rc::new(OnceCell::new()),
                right: Rc::new(OnceCell::new()),
            }
        );

        let l = Rc::clone(&root.get().unwrap().left);
        let _ = l.set(
            Node { 
                loc: String::from("L"),
                parent: Rc::downgrade(&root),
                left: Rc::new(OnceCell::new()),
                right: Rc::new(OnceCell::new()),
            }
        );

        let rl = Rc::clone(&r.get().unwrap().left);
        let _ = rl.set(
            Node { 
                loc: String::from("RL"),
                parent: Rc::downgrade(&r),
                left: Rc::new(OnceCell::new()),
                right: Rc::new(OnceCell::new()),
            }
        );

        let rlr = Rc::clone(&rl.get().unwrap().right);
        let _ = rlr.set(
            Node { 
                loc: String::from("RLR"),
                parent: Rc::downgrade(&rl),
                left: Rc::new(OnceCell::new()),
                right: Rc::new(OnceCell::new()),
            }
        );

        let mut nodes = HashMap::new();
        nodes.insert(String::from(""), root);
        nodes.insert(String::from("R"), r);
        nodes.insert(String::from("L"), l);
        nodes.insert(String::from("RL"), rl);
        nodes.insert(String::from("RLR"), rlr);
        (tree, nodes)
    }

    #[test]
    fn new_trees_have_no_root() {
        let tree = BinaryTree::new();
        assert!(tree.root.get().is_none());
    }

    #[test]
    fn push_empty_tree_adds_root() {
        let tree = BinaryTree { root: Rc::new(OnceCell::new()) };
        tree.push(String::new());
        assert!(tree.root.get().is_some());
    }

    #[test]
    fn depth_recursive_returns() {
        let (_, nodes) = build_test_tree();
        let root = nodes.get("").unwrap();
        let r = nodes.get("R").unwrap();
        let l = nodes.get("L").unwrap();
        let rl = nodes.get("RL").unwrap();
        let rlr = nodes.get("RLR").unwrap();

        assert_eq!(BinaryTree::depth_recursive(root), 0);
        assert_eq!(BinaryTree::depth_recursive(r), 1);
        assert_eq!(BinaryTree::depth_recursive(l), 1);
        assert_eq!(BinaryTree::depth_recursive(rl), 2);
        assert_eq!(BinaryTree::depth_recursive(rlr), 3);
    }

    #[test]
    fn depth_iterative_returns() {
        let (_, nodes) = build_test_tree();
        let root = nodes.get("").unwrap();
        let r = nodes.get("R").unwrap();
        let l = nodes.get("L").unwrap();
        let rl = nodes.get("RL").unwrap();
        let rlr = nodes.get("RLR").unwrap();

        assert_eq!(BinaryTree::depth_iterative(root), 0);
        assert_eq!(BinaryTree::depth_iterative(r), 1);
        assert_eq!(BinaryTree::depth_iterative(l), 1);
        assert_eq!(BinaryTree::depth_iterative(rl), 2);
        assert_eq!(BinaryTree::depth_iterative(rlr), 3);
    }

    #[test]
    fn size_recursive_returns() {
        let (_, nodes) = build_test_tree();
        let root = nodes.get("").unwrap();
        let r = nodes.get("R").unwrap();
        let l = nodes.get("L").unwrap();
        let rl = nodes.get("RL").unwrap();
        let rlr = nodes.get("RLR").unwrap();

        assert_eq!(BinaryTree::size_recursive(root), 5);
        assert_eq!(BinaryTree::size_recursive(r), 3);
        assert_eq!(BinaryTree::size_recursive(l), 1);
        assert_eq!(BinaryTree::size_recursive(rl), 2);
        assert_eq!(BinaryTree::size_recursive(rlr), 1);
    }
}
