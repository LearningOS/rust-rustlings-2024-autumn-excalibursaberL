/*
	binary_search tree
	This problem requires you to implement a basic interface for a binary tree
*/

use std::cmp::Ordering;
use std::fmt::Debug;


#[derive(Debug)]
struct TreeNode<T>
where
    T: Ord+Copy,
{
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

#[derive(Debug)]
struct BinarySearchTree<T>
where
    T: Ord+Copy,
{
    root: Option<Box<TreeNode<T>>>,
}

impl<T:Ord+Copy> TreeNode<T>
where
    T: Ord+Copy,
{
    fn new(value: T) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }
}

impl<T:Ord+Copy> BinarySearchTree<T>
where
    T: Ord+Copy,
{

    fn new() -> Self {
        BinarySearchTree { root: None }
    }

    // Insert a value into the BST
    fn insert_node(node :Option<Box<TreeNode<T>>>,value: T)->Box<TreeNode<T>>{
        match node {
            Some(mut node)=>{
                match node.value.cmp(&value) {
                    Ordering::Less=>{
                        node.right = Some(Self::insert_node(node.right.take(), value));
                    },
                    Ordering::Greater=>{node.left = Some(Self::insert_node(node.left.take(),value));
                    },
                    Ordering::Equal=>{}
                }
                node
            }
            None=>Box::new(TreeNode::new(value))
        }
    }
    fn insert(&mut self, value: T) {
        //TODO
        self.root=Some(Self::insert_node(self.root.take(), value));
    }

    fn search_node(node:Option<&Box<TreeNode<T>>>,value: T)->bool{
        match node {
            Some(n)=>{
                match value.cmp(&n.value) {
                    Ordering::Equal=>true,
                    Ordering::Greater=>Self::search_node(n.right.as_ref(),value),
                    Ordering::Less=>Self::search_node(n.left.as_ref(), value),
                }
            }
            None=>false,
        }
    }
    // Search for a value in the BST
    fn search(&self, value: T) -> bool {
        //TODO
        Self::search_node(self.root.as_ref(), value)
    }
}

impl<T> TreeNode<T>
where
    T: Ord+Copy,
{
    // Insert a node into the tree
    fn insert(&mut self, value: T) {
        //TODO

    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_search() {
        let mut bst = BinarySearchTree::new();

        
        assert_eq!(bst.search(1), false);

        
        bst.insert(5);
        bst.insert(3);
        bst.insert(7);
        bst.insert(2);
        bst.insert(4);

        
        assert_eq!(bst.search(5), true);
        assert_eq!(bst.search(3), true);
        assert_eq!(bst.search(7), true);
        assert_eq!(bst.search(2), true);
        assert_eq!(bst.search(4), true);

        
        assert_eq!(bst.search(1), false);
        assert_eq!(bst.search(6), false);
    }

    #[test]
    fn test_insert_duplicate() {
        let mut bst = BinarySearchTree::new();

        
        bst.insert(1);
        bst.insert(1);

        
        assert_eq!(bst.search(1), true);

        
        match bst.root {
            Some(ref node) => {
                assert!(node.left.is_none());
                assert!(node.right.is_none());
            },
            None => panic!("Root should not be None after insertion"),
        }
    }
}    


