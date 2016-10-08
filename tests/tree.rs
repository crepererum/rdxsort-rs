extern crate rdxsort;

use rdxsort::*;

#[test]
fn test_insert() {
    let mut tree: RdxTree<u32> = RdxTree::new();
    tree.insert(1);
    tree.insert(22);
    tree.insert(2);
    tree.insert(1024);
    tree.insert(0);

    let should = vec![0, 1, 2, 22, 1024];
    let is: Vec<u32> = tree.iter().collect();
    assert_eq!(should, is);
    assert_eq!(tree.nnodes(), (4, 3, 5, 56));
}

#[test]
fn test_insert_float() {
    let mut tree: RdxTree<f32> = RdxTree::new();
    tree.insert(1f32);
    tree.insert(22f32);
    tree.insert(2f32);
    tree.insert(-1024f32);
    tree.insert(-1f32);
    tree.insert(1024f32);
    tree.insert(0f32);

    let should = vec![-1024f32, -1f32, 0f32, 1f32, 2f32, 22f32, 1024f32];
    let is: Vec<f32> = tree.iter().collect();
    assert_eq!(should, is);
    assert_eq!(tree.nnodes(), (4, 7, 7, 54));
}
