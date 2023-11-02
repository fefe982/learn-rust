// https://leetcode.com/problems/count-nodes-equal-to-average-of-subtree/
// 2265. Count Nodes Equal to Average of Subtree
pub struct Solution;
use super::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    fn avg(root: &Option<Rc<RefCell<TreeNode>>>) -> (i32, i32, i32) {
        if let Some(node) = root {
            let node = node.borrow();
            let left = Self::avg(&node.left);
            let right = Self::avg(&node.right);
            let sum = left.0 + right.0 + node.val;
            let count = left.1 + right.1 + 1;
            (
                sum,
                count,
                left.2 + right.2 + if sum / count == node.val { 1 } else { 0 },
            )
        } else {
            (0, 0, 0)
        }
    }
    pub fn average_of_subtree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Solution::avg(&root).2
    }
}
#[cfg(test)]
mod tests {
    use super::super::binary_tree::NULL;
    use super::*;
    #[test]
    fn test_average_of_subtree() {
        let null = NULL;
        assert_eq!(
            Solution::average_of_subtree(TreeNode::from_vec(vec![4, 8, 5, 0, 1, null, 6])),
            5
        );
        assert_eq!(Solution::average_of_subtree(TreeNode::from_vec(vec![1])), 1);
    }
}
