// https://leetcode.com/problems/sum-of-left-leaves/
// 404. Sum of Left Leaves
pub struct Solution;
use super::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    fn sum(root: Option<Rc<RefCell<TreeNode>>>, left: bool) -> i32 {
        if let Some(node) = root {
            let node = node.borrow();
            if left && node.left.is_none() && node.right.is_none() {
                return node.val;
            }
            Solution::sum(node.left.clone(), true) + Solution::sum(node.right.clone(), false)
        } else {
            0
        }
    }
    pub fn sum_of_left_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Solution::sum(root, false)
    }
}
#[cfg(test)]
mod tests {
    use super::super::binary_tree::NULL;
    use super::*;
    #[test]
    fn test_sum_of_left_leaves() {
        let null = NULL;
        assert_eq!(
            Solution::sum_of_left_leaves(TreeNode::from_vec(vec![3, 9, 20, null, null, 15, 7])),
            24
        );
        assert_eq!(Solution::sum_of_left_leaves(TreeNode::from_vec(vec![1])), 0);
    }
}
