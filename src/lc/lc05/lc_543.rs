// https://leetcode.com/problems/diameter-of-binary-tree/
// 543. Diameter of Binary Tree
pub struct Solution;
use super::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    fn get_diameter(root: Option<Rc<RefCell<TreeNode>>>) -> (i32, i32) {
        match root {
            None => (0, 0),
            Some(node) => {
                let left = Self::get_diameter(node.borrow().left.clone());
                let right = Self::get_diameter(node.borrow().right.clone());
                (left.0.max(right.0).max(left.1 + right.1 + 1), left.1.max(right.1) + 1)
            }
        }
    }
    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::get_diameter(root).0 - 1
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_diameter_of_binary_tree() {
        assert_eq!(
            Solution::diameter_of_binary_tree(TreeNode::from_vec(vec![1, 2, 3, 4, 5])),
            3
        );
        assert_eq!(Solution::diameter_of_binary_tree(TreeNode::from_vec(vec![1, 2])), 1);
    }
}
