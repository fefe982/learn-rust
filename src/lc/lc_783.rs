// https://leetcode.com/problems/minimum-absolute-difference-in-bst/
// 783. Minimum Distance Between BST Nodes
pub struct Solution;
use super::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn min_diff_in_bst(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        super::lc_530::Solution::get_minimum_difference(root)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn min_diff_in_bst() {
        let null = super::super::binary_tree::NULL;
        assert_eq!(Solution::min_diff_in_bst(TreeNode::from_vec(vec![4, 2, 6, 1, 3])), 1);
        assert_eq!(
            Solution::min_diff_in_bst(TreeNode::from_vec(vec![1, 0, 48, null, null, 12, 49])),
            1
        );
    }
}
