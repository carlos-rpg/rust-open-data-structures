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

    pub fn push(&self, loc: String) -> &Node {
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
        link.get().unwrap()
    }

    pub fn depth_recursive(node: &Node) -> usize {
        match node.parent.upgrade() {
            None => 0,
            Some(link) => {
                let parent_node = link.get().unwrap();
                1 + Self::depth_recursive(parent_node)
            },
        }
    }

    pub fn depth_iterative(node: &Node) -> usize {
        let mut depth = 0;
        let mut link_parent = node.parent.upgrade();

        while let Some(link) = link_parent {
            link_parent = link.get()
                .expect("Parent node should be initialized")
                .parent
                .upgrade();

            depth += 1;
        }
        depth
    }
    pub fn size_recursive(node: &Node) -> usize {
        match (node.left.get(), node.right.get()) {
            (None, None) => 0,
            (None, Some(r)) => 1 + Self::size_recursive(r),
            (Some(l), None) => 1 + Self::size_recursive(l),
            (Some(l), Some(r)) => 2 + 
                Self::size_recursive(l) +
                Self::size_recursive(r),
        }
    }

    pub fn size_iterative(link: &Link) -> usize {
        unimplemented!()
    }

    pub fn height_recursive(node: &Node) -> usize {
        match (node.left.get(), node.right.get()) {
            (None, None) => 0,
            (None, Some(right)) => 1 + Self::height_recursive(right),
            (Some(left), None) => 1 + Self::height_recursive(left),
            (Some(left), Some(right)) => 1 + usize::max(
                Self::height_recursive(left),
                Self::height_recursive(right),
            ),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    fn build_test_tree() -> (BinaryTree, HashMap<String, Link>) {
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

        let mut links = HashMap::new();
        links.insert(String::from(""), root);
        links.insert(String::from("R"), r);
        links.insert(String::from("L"), l);
        links.insert(String::from("RL"), rl);
        links.insert(String::from("RLR"), rlr);
        (tree, links)
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
        let (_, links) = build_test_tree();
        let root = links[""].get().unwrap();
        let r = links["R"].get().unwrap();
        let l = links["L"].get().unwrap();
        let rl = links["RL"].get().unwrap();
        let rlr = links["RLR"].get().unwrap();

        assert_eq!(BinaryTree::depth_recursive(root), 0);
        assert_eq!(BinaryTree::depth_recursive(r), 1);
        assert_eq!(BinaryTree::depth_recursive(l), 1);
        assert_eq!(BinaryTree::depth_recursive(rl), 2);
        assert_eq!(BinaryTree::depth_recursive(rlr), 3);
    }

    #[test]
    fn depth_iterative_returns() {
        let (_, links) = build_test_tree();
        let root = links[""].get().unwrap();
        let r = links["R"].get().unwrap();
        let l = links["L"].get().unwrap();
        let rl = links["RL"].get().unwrap();
        let rlr = links["RLR"].get().unwrap();

        assert_eq!(BinaryTree::depth_iterative(root), 0);
        assert_eq!(BinaryTree::depth_iterative(r), 1);
        assert_eq!(BinaryTree::depth_iterative(l), 1);
        assert_eq!(BinaryTree::depth_iterative(rl), 2);
        assert_eq!(BinaryTree::depth_iterative(rlr), 3);
    }

    #[test]
    fn size_recursive_returns() {
        let (_, links) = build_test_tree();
        let root = links[""].get().unwrap();
        let r = links["R"].get().unwrap();
        let l = links["L"].get().unwrap();
        let rl = links["RL"].get().unwrap();
        let rlr = links["RLR"].get().unwrap();

        assert_eq!(BinaryTree::size_recursive(root), 4);
        assert_eq!(BinaryTree::size_recursive(r), 2);
        assert_eq!(BinaryTree::size_recursive(l), 0);
        assert_eq!(BinaryTree::size_recursive(rl), 1);
        assert_eq!(BinaryTree::size_recursive(rlr), 0);
    }

    #[test]
    fn height_recursive_returns() {
        let (_, links) = build_test_tree();
        let root = links[""].get().unwrap();
        let r = links["R"].get().unwrap();
        let l = links["L"].get().unwrap();
        let rl = links["RL"].get().unwrap();
        let rlr = links["RLR"].get().unwrap();

        assert_eq!(BinaryTree::height_recursive(root), 3);
        assert_eq!(BinaryTree::height_recursive(r), 2);
        assert_eq!(BinaryTree::height_recursive(l), 0);
        assert_eq!(BinaryTree::height_recursive(rl), 1);
        assert_eq!(BinaryTree::height_recursive(rlr), 0);
    }
}
