// https://leetcode.com/problems/amount-of-time-for-binary-tree-to-be-infected/
// 2385. Amount of Time for Binary Tree to Be Infected
pub struct Solution;
use super::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    fn helper(root: &Option<Rc<RefCell<TreeNode>>>, start: i32) -> (i32, i32, i32) {
        if let Some(node) = root {
            let node = node.borrow();
            let (lup, ldown, l) = Self::helper(&node.left, start);
            let (rup, rdown, r) = Self::helper(&node.right, start);
            if node.val == start {
                (1, 0, ldown.max(rdown))
            } else {
                let up = lup.max(rup);
                if up == 0 {
                    (0, ldown.max(rdown) + 1, 0)
                } else {
                    (up + 1, 0, (ldown + rup).max(rdown + lup).max(l).max(r))
                }
            }
        } else {
            (0, 0, 0)
        }
    }
    pub fn amount_of_time(root: Option<Rc<RefCell<TreeNode>>>, start: i32) -> i32 {
        Solution::helper(&root, start).2
    }
}
#[cfg(test)]
mod tests {
    use super::super::binary_tree::NULL;
    use super::*;
    #[test]
    fn test_amount_of_time() {
        let null = NULL;
        assert_eq!(
            Solution::amount_of_time(
                TreeNode::from_vec(vec![6, 5, null, 12, 2, null, null, 3, 18, null, null, null, 20]),
                3
            ),
            3
        );
        assert_eq!(
            Solution::amount_of_time(TreeNode::from_vec(vec![1, 5, 3, null, 4, 10, 6, 9, 2]), 3),
            4
        );
        assert_eq!(Solution::amount_of_time(TreeNode::from_vec(vec![1]), 1), 0);
    }
}
