// https://leetcode.com/problems/binary-search-tree-to-greater-sum-tree/
// 1038. Binary Search Tree to Greater Sum Tree
pub struct Solution;
use super::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    fn bst_to_gst_sum(root: Option<Rc<RefCell<TreeNode>>>, sum: i32) -> (Option<Rc<RefCell<TreeNode>>>, i32) {
        if let Some(root) = root {
            let (right, sum) = Self::bst_to_gst_sum(root.borrow_mut().right.take(), sum);
            root.borrow_mut().val += sum;
            let sum = root.borrow().val;
            root.borrow_mut().right = right;
            let (left, sum) = Self::bst_to_gst_sum(root.borrow_mut().left.take(), sum);
            root.borrow_mut().left = left;
            (Some(root), sum)
        } else {
            (None, sum)
        }
    }
    pub fn bst_to_gst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        Solution::bst_to_gst_sum(root, 0).0
    }
}
#[cfg(test)]
mod tests {
    use super::super::binary_tree::NULL;
    use super::*;
    #[test]
    fn test_bst_to_gst() {
        let null = NULL;
        assert_eq!(
            Solution::bst_to_gst(TreeNode::from_vec(vec![
                4, 1, 6, 0, 2, 5, 7, null, null, null, 3, null, null, null, 8
            ])),
            TreeNode::from_vec(vec![
                30, 36, 21, 36, 35, 26, 15, null, null, null, 33, null, null, null, 8
            ])
        );
        assert_eq!(
            Solution::bst_to_gst(TreeNode::from_vec(vec![0, null, 1])),
            TreeNode::from_vec(vec![1, null, 1])
        );
    }
}
