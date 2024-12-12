
mod avl;
mod bst;


fn main() {
  let mut avl = avl::AVLTree::new();
  avl.insert(5);
  avl.insert(3);
  avl.insert(7);
  avl.insert(10);
  avl.insert(20);
  println!("{:?}", avl.in_order_traversal());
  avl.remove(3);
  println!("{:?}", avl.in_order_traversal());

  let mut bst = bst::BST::new();
  bst.insert(5);
  bst.insert(3);
  bst.insert(7);
  bst.insert(10);
  bst.insert(20);
  println!("{:?}", bst.in_order_traversal());
  bst.remove(3);
  println!("{:?}", bst.in_order_traversal());
}