/// AVL树（Adelson-Velsky and Landis Tree）是一种自平衡二叉搜索树（Binary Search Tree, BST），
/// 以其发明者G.M. Adelson-Velsky 和 E.M. Landis 命名。它是计算机科学中一种用于保持数据高效排序的基础数据结构。
/// AVL树的特点是任何节点的两个子树的高度差（平衡因子）最多为1，因此始终保持平衡，从而保证了操作的效率
use std::cmp::max;

pub struct Node<T: Ord + Clone> {
  value: T,
  height: usize,
  left: Link<T>,
  right: Link<T>,
}

// type alias
type Link<T> = Option<Box<Node<T>>>;

impl<T: Ord + Clone> Node<T> {
  fn new(value: T) -> Self {
    Node { value, height: 1, left: None, right: None }
  }

  fn height(node: &Link<T>) -> usize {
    node.as_ref().map_or(0, |n| n.height)
  }
  fn balance_factor(&self) -> isize {
    Node::height(&self.left) as isize - Node::height(&self.right) as isize
  }
  fn update_height(&mut self) {
    self.height = 1 + max(Node::height(&self.left), Node::height(&self.right))
  }

  fn rotate_right(mut self: Box<Self>) -> Box<Self> {
    let mut new_root = self.left.take().unwrap();
    self.left = new_root.right.take();
    self.update_height();
    new_root.right = Some(self);
    new_root.update_height();
    new_root
  }

  fn rotate_left(mut self: Box<Self>) -> Box<Self> {
    let mut new_root = self.right.take().unwrap();
    self.right = new_root.left.take();
    self.update_height();
    new_root.left = Some(self);
    new_root.update_height();
    new_root
  }
  fn balance(mut self: Box<Self>) -> Box<Self> {
    self.update_height();
    match self.balance_factor() {
      2 => {
        if self.left.as_ref().unwrap().balance_factor() < 0 {
          self.left = Some(self.left.unwrap().rotate_left());
        }
        self.rotate_right()
      }
      -2 => {
        if self.right.as_ref().unwrap().balance_factor() > 0 {
          self.right = Some(self.right.unwrap().rotate_right())
        }
        self.rotate_left()
      }
      _ => self,
    }
  }
}

pub struct AVLTree<T: Ord + Clone> {
  root: Link<T>,
}

impl<T: Ord + Clone> AVLTree<T> {
  pub fn new() -> Self {
    AVLTree { root: None }
  }
  pub fn insert(&mut self, value: T) {
    self.root = Self::insert_internal(self.root.take(), value);
  }

  fn insert_internal(node: Link<T>, value: T) -> Link<T> {
    if let Some(mut n) = node {
      if value < n.value {
        n.left = Self::insert_internal(n.left.take(), value);
      } else if value > n.value {
        n.right = Self::insert_internal(n.right.take(), value);
      } else {
        n.value = value;
      }
      Some(n.balance())
    } else {
      Some(Box::new(Node::new(value)))
    }
  }
  pub fn contains(&self, value: T) -> bool {
    let mut cur = &self.root;
    while let Some(node) = cur {
      if value < node.value {
        cur = &node.left;
      } else if value > node.value {
        cur = &node.right;
      } else {
        return true;
      }
    }
    false
  }

  pub fn remove(&mut self, value: T) {
    self.root = Self::remove_internal(self.root.take(), value);
  }

  fn remove_internal(node: Link<T>, value: T) -> Link<T> {
    if let Some(mut n) = node {
      if value < n.value {
        n.left = Self::remove_internal(n.left.take(), value);
      } else if value > n.value {
        n.right = Self::remove_internal(n.right.take(), value);
      } else {
        if n.left.is_none() {
          return n.right;
        }
        if n.right.is_none() {
          return n.left;
        }
        let mut successor = n.right.take().unwrap();
        let mut parent = &mut successor;
        while let Some(ref mut left) = parent.left {
          parent = left;
        }
        // 替换当前节点的值为后继节点的值
        n.value = parent.value.clone();
        n.right = Self::remove_internal(Some(successor), n.value.clone());
      }
      Some(n.balance())
    } else {
      None
    }
  }

  pub fn in_order_traversal(&self) -> Vec<T> {
    let mut result = Vec::new();
    Self::in_order_helper(&self.root, &mut result);
    result
  }

  fn in_order_helper(node: &Link<T>, result: &mut Vec<T>) {
    if let Some(n) = node {
      Self::in_order_helper(&n.left, result);
      result.push(n.value.clone());
      Self::in_order_helper(&n.right, result);
    }
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_avl_tree() {
    let mut tree = AVLTree::new();
    tree.insert(2);
    tree.insert(1);
    tree.insert(3);
    assert!(tree.contains(3));
    assert!(!tree.contains(4));
    assert!(tree.in_order_traversal() == vec![1, 2, 3]);
    tree.remove(3);
    assert!(!tree.contains(3));
    assert!(tree.in_order_traversal() == vec![1, 2]);
    assert!(tree.contains(2))
  }
}
