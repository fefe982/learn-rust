// https://leetcode.com/problems/minimum-absolute-difference-in-bst/
// 530. Minimum Absolute Difference in BST
pub struct Solution;
use super::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    fn get_min(
        root: &Option<Rc<RefCell<TreeNode>>>,
        last: Option<i32>,
        mut cmin: i32,
    ) -> (Option<i32>, i32) {
        if let Some(node) = root {
            if let (Some(l), nmin) = Self::get_min(&node.borrow().left, last, cmin) {
                cmin = std::cmp::min(nmin, node.borrow().val - l);
            }
            Self::get_min(&node.borrow().right, Some(node.borrow().val), cmin)
        } else {
            (last, cmin)
        }
    }
    pub fn get_minimum_difference(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::get_min(&root, None, i32::MAX).1
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn get_minimum_difference() {
        let null = super::super::binary_tree::NULL;
        assert_eq!(
            Solution::get_minimum_difference(TreeNode::from_vec(vec![4, 2, 6, 1, 3])),
            1
        );
        assert_eq!(
            Solution::get_minimum_difference(TreeNode::from_vec(vec![
                1, 0, 48, null, null, 12, 49
            ])),
            1
        );
    }
}
