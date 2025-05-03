use ods::binary_search_tree::BinarySearchTree;

#[test]
fn binary_search_tree() {
    let mut tree = BinarySearchTree::new();

    assert!(tree.is_empty());

    tree.add(0);
    tree.add(-10);
    tree.add(10);
    tree.add(15);
    tree.add(5);
    let refnode_10 = tree.find(10).expect("10 should be in tree");
    assert_eq!(refnode_10.size(), 3);
    assert_eq!(refnode_10.depth(), 1);
    assert_eq!(refnode_10.height(), 2);

    tree.add(17);
    assert_eq!(refnode_10.height(), 3);

    tree.remove(15);
    let refnode_17 = tree.find(17).expect("17 should be in tree");
    assert_eq!(refnode_17.depth(), 2);
    assert_eq!(refnode_17.size(), 1);

    tree.remove(0);
    assert_eq!(tree.size(), 4);
}