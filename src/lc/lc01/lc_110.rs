// https://leetcode.com/problems/balanced-binary-tree/description/
// 110. Balanced Binary Tree
pub struct Solution;
use super::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    fn balanced(root: &Option<Rc<RefCell<TreeNode>>>) -> (bool, i32) {
        if let Some(node) = root {
            let (left_balanced, left_height) = Self::balanced(&node.borrow().left);
            let (right_balanced, right_height) = Self::balanced(&node.borrow().right);
            if left_balanced && right_balanced && (left_height - right_height).abs() <= 1 {
                return (true, left_height.max(right_height) + 1);
            } else {
                return (false, 0);
            }
        }
        (true, 0)
    }
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::balanced(&root).0
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_is_balanced() {
        let null = super::super::binary_tree::NULL;
        assert_eq!(
            Solution::is_balanced(TreeNode::from_vec(vec![3, 9, 20, null, null, 15, 7])),
            true
        );
        assert_eq!(
            Solution::is_balanced(TreeNode::from_vec(vec![1, 2, 2, 3, 3, null, null, 4, 4])),
            false
        );
    }
}
