// https://leetcode.com/problems/convert-bst-to-greater-tree/
// 538. Convert BST to Greater Tree
pub struct Solution;
use super::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    fn convert(root: &mut Option<Rc<RefCell<TreeNode>>>, sum: i32) -> i32 {
        if let Some(node) = root {
            let mut node = node.borrow_mut();
            let right_sum = Self::convert(&mut node.right, sum);
            node.val += right_sum;
            let val = node.val;
            Self::convert(&mut node.left, val)
        } else {
            sum
        }
    }
    pub fn convert_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut root = root;
        Self::convert(&mut root, 0);
        root
    }
}
#[cfg(test)]
mod tests {
    use super::super::binary_tree::NULL;
    use super::*;
    #[test]
    fn convert_bst() {
        let null = NULL;
        assert_eq!(
            Solution::convert_bst(TreeNode::from_vec(vec![
                4, 1, 6, 0, 2, 5, 7, null, null, null, 3, null, null, null, 8
            ])),
            TreeNode::from_vec(vec![
                30, 36, 21, 36, 35, 26, 15, null, null, null, 33, null, null, null, 8
            ])
        );
        assert_eq!(
            Solution::convert_bst(TreeNode::from_vec(vec![0, null, 1])),
            TreeNode::from_vec(vec![1, null, 1])
        );
    }
}
