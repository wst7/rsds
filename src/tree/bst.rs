// Binary Search Tree implementation in Rust

// BST的性质

// 	1.	节点的值排序：
// 	•	每个节点的左子树中所有节点的值小于当前节点的值。
// 	•	每个节点的右子树中所有节点的值大于当前节点的值。
// 	•	每个子树本身也是一棵二叉搜索树。
// 	2.	无重复值（可选）：
// 	•	通常情况下，BST不允许存储重复值，但可以通过修改设计来支持。

// BST的优缺点
// 优点：
// 	•	实现简单。
// 	•	支持动态集合操作，能够高效地查找、插入和删除。
// 缺点：
// 	•	当插入数据顺序不当时（如升序或降序），会退化为链表，导致操作效率降低。
// 	•	无法自动保持平衡，需要结合平衡机制（如AVL树或红黑树）。

// 改进和扩展
// 	•	平衡二叉搜索树：
// 	•	AVL树、红黑树通过自动平衡机制解决了普通BST退化为链表的问题。
// 	•	B树和B+树：
// 	•	用于外存（如磁盘）的大规模数据存储，适合数据库索引。

#[derive(Clone)]
struct Node<T: Ord + Clone> {
  value: T,
  left: Link<T>,
  right: Link<T>,
}
impl<T: Ord + Clone> Node<T> {
  pub fn new(value: T) -> Self {
    Node { value, left: None, right: None }
  }
}
type Link<T> = Option<Box<Node<T>>>;

pub struct BST<T: Ord + Clone> {
  root: Link<T>,
}

impl<T: Ord + Clone> BST<T> {
  pub fn new() -> Self {
    BST { root: None }
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
      Some(n)
    } else {
      Some(Box::new(Node::new(value)))
    }
  }

  pub fn remove(&mut self, value: T) {
    self.root = Self::remove_internal(self.root.take(), value);
  }
  fn remove_internal(node: Link<T>, value: T) -> Link<T> {
    if let Some(mut n) = node {
      if value < n.value {
        n.left = Self::remove_internal(n.left.take(), value)
      } else if value > n.value {
        n.right = Self::remove_internal(n.right.take(), value)
      } else {
        if n.left.is_none() {
          return n.right;
        }
        if n.right.is_none() {
          return n.left;
        }
        let mut successor = n.right.take().unwrap();
        let mut parent = &mut successor;
        // 找到后继节点,右侧最小的节点
        while let Some(ref mut left) = parent.left {
          parent = left;
        }
        n.value = parent.value.clone();
        n.right = Self::remove_internal(Some(successor), n.value.clone());
        return Some(n);
      }
      return Some(n);
    } else {
      None
    }
  }

  pub fn search(&self, value: T) -> Link<T> {
    Self::search_internal(&self.root, value)
  }
  fn search_internal(node: &Link<T>, value: T) -> Link<T> {
    if let Some(n) = node {
      if value < n.value {
        Self::search_internal(&n.left, value)
      } else if value > n.value {
        Self::search_internal(&n.right, value)
      } else {
        return Some(n.clone());
      }
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
  #[test]
  fn test_bst() {
    use super::*;
    let mut bst = BST::new();
    bst.insert(3);
    bst.insert(4);
    bst.insert(1);
    bst.insert(2);
    bst.insert(5);
    bst.insert(6);
    assert!(bst.search(4).is_some());
    assert_eq!(bst.in_order_traversal(), vec![1, 2, 3, 4, 5, 6]);
    bst.remove(5);
    assert_eq!(bst.in_order_traversal(), vec![1, 2, 3, 4, 6]);
    assert!(bst.search(5).is_none());
  }
}
