// https://leetcode.com/problems/binary-tree-maximum-path-sum/
// 124. Binary Tree Maximum Path Sum
pub struct Solution;
use super::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    fn max_path_sum_ref(root: &Option<Rc<RefCell<TreeNode>>>) -> (i32, i32) {
        if let Some(node) = root {
            let node = node.as_ref().borrow();
            let (max_l, max_l_s) = Self::max_path_sum_ref(&node.left);
            let (max_r, max_r_s) = Self::max_path_sum_ref(&node.right);
            (
                std::cmp::max(std::cmp::max(max_l, max_r), max_l_s + max_r_s + node.val),
                std::cmp::max(0, std::cmp::max(max_l_s, max_r_s) + node.val),
            )
        } else {
            (i32::MIN, 0)
        }
    }
    pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::max_path_sum_ref(&root).0
    }
}
#[cfg(test)]
mod tests {
    use super::super::binary_tree::NULL;
    use super::*;
    #[test]
    fn max_path_sum() {
        assert_eq!(
            Solution::max_path_sum(TreeNode::from_vec(&vec![2, 1, 3])),
            6
        );
        assert_eq!(
            Solution::max_path_sum(TreeNode::from_vec(&vec![-10, 9, 20, NULL, NULL, 15, 7])),
            42
        );
    }
}
