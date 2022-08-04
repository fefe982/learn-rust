// https://leetcode.com/problems/range-sum-of-bst/
// 938. Range Sum of BST
use super::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
pub struct Solution;
impl Solution {
    fn range_sum_bst_helper(
        root: &Option<Rc<RefCell<TreeNode>>>,
        low: i32,
        high: i32,
        b_low: i32,
        b_high: i32,
    ) -> i32 {
        match root {
            None => 0,
            Some(n) => {
                let node = n.borrow();
                if node.val < low {
                    if b_high >= low {
                        Self::range_sum_bst_helper(&node.right, low, high, node.val, b_high)
                    } else {
                        0
                    }
                } else if node.val > high {
                    if b_low <= high {
                        Self::range_sum_bst_helper(&node.left, low, high, b_low, node.val)
                    } else {
                        0
                    }
                } else {
                    node.val
                        + Self::range_sum_bst_helper(&node.left, low, high, b_low, node.val)
                        + Self::range_sum_bst_helper(&node.right, low, high, node.val, b_high)
                }
            }
        }
    }
    pub fn range_sum_bst(root: Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> i32 {
        Self::range_sum_bst_helper(&root, low, high, 0, 200000)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn range_sum_bst() {
        assert_eq!(
            Solution::range_sum_bst(TreeNode::from_vec(&vec![10, 5, 15, 3, 7, -1, 18]), 7, 15),
            32
        );
        assert_eq!(
            Solution::range_sum_bst(
                TreeNode::from_vec(&vec![10, 5, 15, 3, 7, 13, 18, 1, -1, 6]),
                6,
                10
            ),
            23
        );
    }
}
