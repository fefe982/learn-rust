// https://leetcode.com/problems/binary-tree-inorder-traversal/
// 94. Binary Tree Inorder Traversal
pub struct Solution;
use super::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    fn inorder(root: Option<Rc<RefCell<TreeNode>>>, mut v: Vec<i32>) -> Vec<i32> {
        if let Some(node) = root {
            v = Self::inorder(node.borrow_mut().left.take(), v);
            v.push(node.borrow().val);
            v = Self::inorder(node.borrow_mut().right.take(), v);
        }
        v
    }
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        Self::inorder(root, vec![])
    }
}
#[cfg(test)]
mod tests {
    use super::super::binary_tree::NULL;
    use super::*;
    #[test]
    fn test_inorder_traversal() {
        let null = NULL;
        assert_eq!(
            Solution::inorder_traversal(TreeNode::from_vec(vec![1, null, 2, 3])),
            vec![1, 3, 2]
        );
        assert_eq!(Solution::inorder_traversal(TreeNode::from_vec(vec![])), vec![]);
        assert_eq!(Solution::inorder_traversal(TreeNode::from_vec(vec![1])), vec![1]);
    }
}
