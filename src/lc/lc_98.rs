// https://leetcode.cn/problems/validate-binary-search-tree/
// 98. Validate Binary Search Tree
pub struct Solution;
use super::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    fn check_tree(root: &Option<Rc<RefCell<TreeNode>>>, min: i64, max: i64) -> bool {
        if let Some(node) = root {
            let node = node.borrow();
            if node.val as i64 <= min || node.val as i64 >= max {
                return false;
            }
            Self::check_tree(&node.left, min, node.val as i64) && Self::check_tree(&node.right, node.val as i64, max)
        } else {
            true
        }
    }
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::check_tree(&root, i64::MIN, i64::MAX)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_is_valid_bst() {
        let null = super::super::binary_tree::NULL;
        assert_eq!(Solution::is_valid_bst(TreeNode::from_vec(vec![2, 1, 3])), true);
        assert_eq!(
            Solution::is_valid_bst(TreeNode::from_vec(vec![5, 1, 4, null, null, 3, 6])),
            false
        );
    }
}
