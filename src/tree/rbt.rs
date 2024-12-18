// Red-Black tree is a self-balancing binary search tree in which each node contains an extra bit for denoting the color of the node, either red or black.
// A red-black tree satisfies the following properties:

// Red/Black Property: Every node is colored, either red or black.
// Root Property: The root is black.
// Leaf Property: Every leaf (NIL) is black.
// Red Property: If a red node has children then, the children are always black.
// Depth Property: For each node, any simple path from this node to any of its descendant leaf has the same black-depth (the number of black nodes).

// https://github.com/tickbh/rbtree-rs/blob/master/src/lib.rs

enum Color {
  Red,
  Black,
}
struct Node<T: Ord> {
  value: T,
  color: Color,
  left: Link<T>,
  right: Link<T>,
  parent: Option<*mut Node<T>>,
}

impl<T: Ord> Node<T> {
  fn new(value: T, color: Color) -> Self {
    Self {
      value,
      color,
      left: None,
      right: None,
      parent: None,
    }
  }
}

type Link<T> = Option<Box<Node<T>>>;


pub struct RBTree<T: Ord> {
  root: Link<T>,
}

impl<T: Ord> RBTree<T>  {
    pub fn new() -> Self {
      RBTree{ root: None }
    }

    fn rotate_left(&mut self, mut node: Link<T>) {
      let mut n = node.unwrap();
      let mut temp = n.right.take().unwrap();
      n.right = temp.left.take();

      temp.left = Some(n);
     
    }

    pub fn insert(&mut self, value: T) {
      self.root = Self::insert_internal(self.root.take(), value);
      if let Some(ref mut n) = self.root {
        n.color = Color::Black; // 根节点始终为黑色
      }
    }
    fn insert_internal(mut node: Link<T>, value: T) -> Link<T> {
      if let Some(mut n) = node {
        if value < n.value {
          n.left = Self::insert_internal(n.left.take(), value);
        } else if value > n.value {
          n.right = Self::insert_internal(n.right, value);
        }
        let n = Self::fix_violations(n);
        Some(n)
      } else {
        return Some(Box::new(Node::new(value, Color::Red)));
      }
    }

    /**
     * 平衡操作
     */
    fn fix_violations(node: Box<Node<T>>) -> Box<Node<T>> {

    }

    fn is_red(node: &Link<T>) -> bool {
      matches!(node.as_ref().map(|n| &n.color), Some(Color::Red))
    }
}