//! # Binary Tress
//!
//! TODO: Implement Binary Tree Traversal (Inorder, Preorder and Postorder)
//!

use std::ops::AddAssign;

pub enum Node<T: Clone> {
    Leaf(T),
    Branch {
        value: T,
        left: Option<Box<Node<T>>>,
        right: Option<Box<Node<T>>>,
    },
}

impl<T: Clone + AddAssign> Node<T> {
    pub fn inorder_traversal(&self) {
        todo!()
    }

    pub fn preorder_traversal(&self) {
        todo!()
    }

    pub fn postorder_traversal(&self) {
        todo!()
    }

    pub fn sum(&self) -> T {
        match self {
            Node::Leaf(t) => t.clone(),
            Node::Branch { value, left, right } => {
                let mut s = value.clone();
                if let Some(l) = left {
                    s += l.sum();
                }
                if let Some(r) = right {
                    s += r.sum();
                }
                s
            }
        }
    }
}

pub struct BinaryTree<T: Default + Clone> {
    root: Option<Node<T>>,
}

impl<T: Default + Clone + AddAssign> BinaryTree<T> {
    pub fn new(root: Option<Node<T>>) -> Self {
        Self { root }
    }

    pub fn sum(&self) -> T {
        match &self.root {
            Some(n) => n.sum(),
            None => T::default(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_init() {
        let tree: BinaryTree<i32> = BinaryTree::new(Some(Node::Branch {
            value: 52,
            left: Some(Box::new(Node::Branch {
                value: 40,
                left: Some(Box::new(Node::Branch {
                    value: 24,
                    left: None,
                    right: Some(Box::new(Node::Leaf(32))),
                })),
                right: None,
            })),
            right: Some(Box::new(Node::Branch {
                value: 62,
                left: Some(Box::new(Node::Leaf(58))),
                right: Some(Box::new(Node::Leaf(69))),
            })),
        }));

        assert_eq!(69 + 58 + 62 + 32 + 24 + 40 + 52, tree.sum());
    }
}
