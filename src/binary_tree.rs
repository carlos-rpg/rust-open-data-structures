//! Safe nodes and mutable references to nodes for binary tree structures.
//! 
//! Internally, a `Node` implements `Weak` references to their parents and `Rc`
//! `Rc` references to its children. However, all the public funcionality is done 
//! through `RefNode<T>`, which are a wrapper around the well known 
//! `Rc<RefCell<Node<T>>>` structures.

use std::rc::{Rc, Weak};
use std::cell::RefCell;


#[derive(Debug)]
struct Node<T> {
    value: T,
    parent: WeakRefNode<T>,
    left: Option<RefNode<T>>,
    right: Option<RefNode<T>>,
}

impl<T> Node<T> {
    fn new(value: T) -> Node<T> {
        Self { value, parent: WeakRefNode::new(), left: None, right: None }
    }
}

impl<T: PartialEq> PartialEq for Node<T> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl<T: PartialOrd> PartialOrd for Node<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.value.partial_cmp(&other.value)
    }
}


#[derive(Debug)]
struct WeakRefNode<T>(Weak<RefCell<Node<T>>>);

impl<T> WeakRefNode<T> {
    fn new() -> Self {
        Self(Weak::new())
    }

    fn upgrade(&self) -> Option<RefNode<T>> {
        self.0.upgrade().map(|x| RefNode(x))
    }
}


/// A reference to a binary tree node.
#[derive(PartialEq, Debug, PartialOrd)]
pub struct RefNode<T>(Rc<RefCell<Node<T>>>);

impl<T> RefNode<T> {
    /// Create a new node containing `value`, with no children or ancestors. 
    /// Returns a `RefNode` to it.
    /// 
    /// # Examples
    /// 
    /// ```
    /// # use ods::binary_tree::RefNode;
    /// let node = RefNode::new(0);
    /// ```
    pub fn new(value: T) -> Self {
        Self(Rc::new(RefCell::new(Node::new(value))))
    }

    /// Returns `true` if the node referenced is a root node, otherwise `false`.
    /// 
    /// # Examples
    /// 
    /// ```
    /// # use ods::binary_tree::RefNode;
    /// let node = RefNode::new(0);
    /// assert!(node.is_root());
    /// ```
    pub fn is_root(&self) -> bool {
        self.get_parent().is_none()
    }

    /// Returns `true` if the node has both children, otherwise `false`.
    /// 
    /// # Examples
    /// 
    /// ```
    /// # use ods::binary_tree::RefNode;
    /// let node = RefNode::new(0);
    /// assert!(!node.is_fully_branched());
    /// ```
    pub fn is_fully_branched(&self) -> bool {
        self.get_left().is_some() && self.get_right().is_some()
    }

    /// Returns a reference to the parent node, `None` if there are no ancestors.
    /// 
    /// # Examples
    /// 
    /// ```
    /// # use ods::binary_tree::RefNode;
    /// let node = RefNode::new(0);
    /// assert!(node.get_parent().is_none());
    /// ```
    pub fn get_parent(&self) -> Option<RefNode<T>> {
        self.0.borrow().parent.upgrade()
    }

    /// Sets `node` as the new parent.
    /// 
    /// # Examples
    /// 
    /// ```
    /// # use ods::binary_tree::RefNode;
    /// let child_node = RefNode::new(0);
    /// let parent_node = RefNode::new(1);
    /// child_node.set_parent(Some(&parent_node));
    /// ```
    pub fn set_parent(&self, node: Option<&RefNode<T>>) {
        self.0.borrow_mut().parent = match node {
            None => WeakRefNode::new(),
            Some(ref_node) => WeakRefNode(Rc::downgrade(&ref_node.0)),
        };
    }

    /// Returns a reference to the left node, `None` if there is no child.
    /// 
    /// # Examples
    /// 
    /// ```
    /// # use ods::binary_tree::RefNode;
    /// let node = RefNode::new(0);
    /// assert!(node.get_left().is_none());
    /// ```
    pub fn get_left(&self) -> Option<RefNode<T>> {
        self.0.borrow().left.as_ref().map(RefNode::clone)
    }

    /// Sets `node` as the new left child.
    /// 
    /// # Examples
    /// 
    /// ```
    /// # use ods::binary_tree::RefNode;
    /// let parent_node = RefNode::new(0);
    /// let child_node = RefNode::new(1);
    /// parent_node.set_left(Some(&child_node));
    /// ```
    pub fn set_left(&self, node: Option<&RefNode<T>>) {
        self.0.borrow_mut().left = node.map(RefNode::clone);
    }

    /// Returns a reference to the right node, `None` if there is no child.
    /// 
    /// # Examples
    /// 
    /// ```
    /// # use ods::binary_tree::RefNode;
    /// let node = RefNode::new(0);
    /// assert!(node.get_right().is_none());
    /// ```
    pub fn get_right(&self) -> Option<RefNode<T>> {
        self.0.borrow().right.as_ref().map(RefNode::clone)
    }

    /// Sets `node` as the new right child.
    /// 
    /// # Examples
    /// 
    /// ```
    /// # use ods::binary_tree::RefNode;
    /// let parent_node = RefNode::new(0);
    /// let child_node = RefNode::new(1);
    /// parent_node.set_right(Some(&child_node));
    /// ```
    pub fn set_right(&self, node: Option<&RefNode<T>>) {
        self.0.borrow_mut().right = node.map(RefNode::clone);
    }

    /// Returns the value stored in the node if `self` is the only reference to it,
    /// `None` if more than one reference exists.
    /// 
    /// # Examples
    /// 
    /// ```
    /// # use ods::binary_tree::RefNode;
    /// let node = RefNode::new(0);
    /// let value = node.into_inner_value().unwrap();
    /// assert_eq!(value, 0);
    /// ```
    pub fn into_inner_value(self) -> Option<T> {
        Some(Rc::into_inner(self.0)?.into_inner().value)
    }

    /// Sets the value stored in the node. 
    /// 
    /// # Examples
    /// 
    /// ```
    /// # use ods::binary_tree::RefNode;
    /// let node = RefNode::new(0);
    /// node.set(1);
    /// ```
    pub fn set(&self, value: T) {
        self.0.borrow_mut().value = value;
    }

    /// Returns the number of nodes to reach the root.
    /// 
    /// # Examples
    /// 
    /// ```
    /// # use ods::binary_tree::RefNode;
    /// let parent = RefNode::new(0);
    /// assert_eq!(parent.depth(), 0);
    /// 
    /// let leaf = RefNode::new(1);
    /// parent.set_left(Some(&leaf));
    /// leaf.set_parent(Some(&parent));
    /// assert_eq!(leaf.depth(), 1);
    /// ```
    pub fn depth(&self) -> usize {
        let mut depth = 0;
        let mut node_opt = self.get_parent();

        while let Some(node) = node_opt {
            depth += 1;
            node_opt = node.get_parent();
        }
        depth
    }

    /// Returns the number of nodes under `self`, including itself.
    /// 
    /// # Examples
    /// 
    /// ```
    /// # use ods::binary_tree::RefNode;
    /// let parent = RefNode::new(0);
    /// let left = RefNode::new(-1);
    /// let right = RefNode::new(1);
    /// 
    /// parent.set_left(Some(&left));
    /// parent.set_right(Some(&right));
    /// left.set_parent(Some(&parent));
    /// right.set_parent(Some(&parent));
    /// 
    /// assert_eq!(parent.size(), 3);
    /// ```
    pub fn size(&self) -> usize {
        let mut size = 0;
        let mut nodes = vec![RefNode::clone(&self)];

        while !nodes.is_empty() {
            size += 1;
            let node = nodes.remove(0);

            if let Some(left_node) = node.get_left() {
                nodes.push(left_node);
            }
            if let Some(right_node) = node.get_right() {
                nodes.push(right_node);
            }
        }
        size
    }

    /// Returns the maximum distance from `self` to any of the leafs under it. 
    /// This implementation is recursive, and therefore there is a risk of panic 
    /// if the tree is too large.
    /// 
    /// # Examples
    /// 
    /// ```
    /// # use ods::binary_tree::RefNode;
    /// let parent = RefNode::new(0);
    /// let left = RefNode::new(-1);
    /// let right = RefNode::new(1);
    /// 
    /// parent.set_left(Some(&left));
    /// parent.set_right(Some(&right));
    /// left.set_parent(Some(&parent));
    /// right.set_parent(Some(&parent));
    /// 
    /// assert_eq!(parent.height(), 2);
    /// ```
    pub fn height(&self) -> usize {
        fn recurse<T>(node_opt: Option<RefNode<T>>) -> usize {
            match node_opt {
                None => 0,
                Some(node) => 1 + usize::max(
                    recurse(node.get_left()), 
                    recurse(node.get_right()),
                )
            }
        }
        recurse(Some(self).cloned())
    }
}

impl<T> Clone for RefNode<T> {
    fn clone(&self) -> Self {
        RefNode(Rc::clone(&self.0))
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    fn build_test_node<T>(value: T) -> RefNode<T> {
        RefNode(Rc::new(RefCell::new(
            Node { value, parent: WeakRefNode(Weak::new()), left: None, right: None }
        )))
    }

    fn build_test_nodes() -> HashMap<String, RefNode<char>> {
        let root = build_test_node('a');
        let l = build_test_node('b');
        let r = build_test_node('c');
        let rl = build_test_node('d');
        let rll = build_test_node('e');
        let rlr = build_test_node('f');

        root.0.borrow_mut().left = Some(RefNode::clone(&l));
        root.0.borrow_mut().right = Some(RefNode::clone(&r));

        l.0.borrow_mut().parent = WeakRefNode(Rc::downgrade(&root.0));

        r.0.borrow_mut().parent = WeakRefNode(Rc::downgrade(&root.0));
        r.0.borrow_mut().left = Some(RefNode::clone(&rl));

        rl.0.borrow_mut().parent = WeakRefNode(Rc::downgrade(&r.0));
        rl.0.borrow_mut().left = Some(RefNode::clone(&rll));
        rl.0.borrow_mut().right = Some(RefNode::clone(&rlr));

        rll.0.borrow_mut().parent = WeakRefNode(Rc::downgrade(&rl.0));
        rlr.0.borrow_mut().parent = WeakRefNode(Rc::downgrade(&rl.0));

        let mut nodes = HashMap::new();
        nodes.insert(String::from(""), root);
        nodes.insert(String::from("R"), r);
        nodes.insert(String::from("L"), l);
        nodes.insert(String::from("RL"), rl);
        nodes.insert(String::from("RLL"), rll);
        nodes.insert(String::from("RLR"), rlr);

        nodes
    }

    #[test]
    fn new_refnode() {
        let node = RefNode::new('a');
        assert!(node.0.borrow().left.is_none());
        assert!(node.0.borrow().right.is_none());
        assert!(node.0.borrow().parent.0.upgrade().is_none());
        assert_eq!(node.0.borrow().value, 'a');
    }

    #[test]
    fn get_parent_no_node() {
        let node = build_test_node('a');
        assert!(node.get_parent().is_none());
    }

    #[test]
    fn get_parent_some_node() {
        let node1 = build_test_node('a');
        let node2 = build_test_node('b');
        node2.0.borrow_mut().parent = WeakRefNode(Rc::downgrade(&node1.0));
        assert!(node2.get_parent().is_some());
        assert_eq!(node2.get_parent().unwrap(), node1);
    }

    #[test]
    fn set_parent() {
        let node1 = build_test_node('a');
        let node2 = build_test_node('b');
        node2.set_parent(Some(&node1));
        assert_eq!(node2.0.borrow().parent.upgrade().unwrap(), node1);
    }
    #[test]
    fn depth_returns() {
        let nodes = build_test_nodes();
        assert_eq!(nodes[""].depth(), 0);
        assert_eq!(nodes["L"].depth(), 1);
        assert_eq!(nodes["R"].depth(), 1);
        assert_eq!(nodes["RL"].depth(), 2);
        assert_eq!(nodes["RLL"].depth() ,3);
        assert_eq!(nodes["RLR"].depth(), 3);
    }

    #[test]
    fn size_returns() {
        let nodes = build_test_nodes();
        assert_eq!(nodes[""].size(), 6);
        assert_eq!(nodes["L"].size(), 1);
        assert_eq!(nodes["R"].size(), 4);
        assert_eq!(nodes["RL"].size(), 3);
        assert_eq!(nodes["RLL"].size(), 1);
        assert_eq!(nodes["RLR"].size(), 1);
    }

    #[test]
    fn height_returns() {
        let nodes = build_test_nodes();
        assert_eq!(nodes[""].height(), 4);
        assert_eq!(nodes["L"].height(), 1);
        assert_eq!(nodes["R"].height(), 3);
        assert_eq!(nodes["RL"].height(), 2);
        assert_eq!(nodes["RLL"].height(), 1);
        assert_eq!(nodes["RLR"].height(), 1);
    }
}
