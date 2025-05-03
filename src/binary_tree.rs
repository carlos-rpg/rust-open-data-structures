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
        Self { value, parent: WeakRefNode(Weak::new()), left: None, right: None }
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
    fn upgrade(&self) -> Option<RefNode<T>> {
        self.0.upgrade().map(|x| RefNode(x))
    }
}


#[derive(PartialEq, Debug, PartialOrd)]
pub struct RefNode<T>(Rc<RefCell<Node<T>>>);

impl<T> RefNode<T> {
    pub fn new(value: T) -> Self {
        Self(Rc::new(RefCell::new(Node::new(value))))
    }

    pub fn is_root(&self) -> bool {
        self.get_parent().is_none()
    }

    pub fn is_fully_branched(&self) -> bool {
        self.get_left().is_some() && self.get_right().is_some()
    }

    pub fn get_parent(&self) -> Option<RefNode<T>> {
        self.0.borrow().parent.upgrade()
    }

    pub fn set_parent(&self, node: &RefNode<T>) {
        self.0.borrow_mut().parent = WeakRefNode(Rc::downgrade(&node.0));
    }

    pub fn get_left(&self) -> Option<RefNode<T>> {
        self.0.borrow().left.as_ref().map(RefNode::clone)
    }

    pub fn set_left(&self, node: Option<&RefNode<T>>) {
        self.0.borrow_mut().left = node.map(RefNode::clone);
    }

    pub fn get_right(&self) -> Option<RefNode<T>> {
        self.0.borrow().right.as_ref().map(RefNode::clone)
    }

    pub fn set_right(&self, node: Option<&RefNode<T>>) {
        self.0.borrow_mut().right = node.map(RefNode::clone);
    }

    pub fn into_inner_value(self) -> Option<T> {
        Some(Rc::into_inner(self.0)?.into_inner().value)
    }

    pub(super) fn set(&self, value: T) {
        self.0.borrow_mut().value = value;
    }

    pub fn depth(&self) -> usize {
        let mut depth = 0;
        let mut node_opt = self.get_parent();

        while let Some(node) = node_opt {
            depth += 1;
            node_opt = node.get_parent();
        }
        depth
    }

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
        node2.set_parent(&node1);
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
