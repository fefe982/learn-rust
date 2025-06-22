// https://leetcode.cn/problems/path-sum/
// 112. Path Sum
pub struct Solution;
use super::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        if let Some(node) = root {
            let node = node.borrow();
            let target_sum = target_sum - node.val;
            if node.left.is_none() && node.right.is_none() {
                return target_sum == 0;
            }
            Self::has_path_sum(node.left.clone(), target_sum) || Self::has_path_sum(node.right.clone(), target_sum)
        } else {
            false
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_has_path_sum() {
        let null = super::super::binary_tree::NULL;
        assert_eq!(
            Solution::has_path_sum(
                TreeNode::from_vec(vec![5, 4, 8, 11, null, 13, 4, 7, 2, null, null, null, 1]),
                22
            ),
            true
        );
        assert_eq!(Solution::has_path_sum(TreeNode::from_vec(vec![1, 2, 3]), 5), false);
        assert_eq!(Solution::has_path_sum(TreeNode::from_vec(vec![]), 0), false);
    }
}
