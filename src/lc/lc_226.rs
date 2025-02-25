// https://leetcode.com/problems/invert-binary-tree/
// 226. Invert Binary Tree
pub struct Solution;
use super::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(node) = root {
            let left = node.borrow_mut().left.take();
            let right = node.borrow_mut().right.take();
            node.borrow_mut().left = Self::invert_tree(right);
            node.borrow_mut().right = Self::invert_tree(left);
            Some(node)
        } else {
            None
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_invert_tree() {
        assert_eq!(
            Solution::invert_tree(TreeNode::from_vec(vec![4, 2, 7, 1, 3, 6, 9])),
            TreeNode::from_vec(vec![4, 7, 2, 9, 6, 3, 1])
        );
        assert_eq!(
            Solution::invert_tree(TreeNode::from_vec(vec![2, 1, 3])),
            TreeNode::from_vec(vec![2, 3, 1])
        );
        assert_eq!(
            Solution::invert_tree(TreeNode::from_vec(vec![])),
            TreeNode::from_vec(vec![])
        );
    }
}
