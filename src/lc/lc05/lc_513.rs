// https://leetcode.com/problems/find-bottom-left-tree-value/
// 513. Find Bottom Left Tree Value
pub struct Solution;
use super::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    fn find_value(root: Option<Rc<RefCell<TreeNode>>>, depth: i32, max_depth: &mut i32, res: &mut i32) {
        if let Some(node) = root {
            let node = node.borrow();
            Self::find_value(node.left.clone(), depth + 1, max_depth, res);
            Self::find_value(node.right.clone(), depth + 1, max_depth, res);
            if depth > *max_depth {
                *max_depth = depth;
                *res = node.val;
            }
        }
    }
    pub fn find_bottom_left_value(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut max_depth = -1;
        let mut res = 0;
        Self::find_value(root, 0, &mut max_depth, &mut res);
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_find_bottom_left_value() {
        let null = super::super::binary_tree::NULL;
        assert_eq!(Solution::find_bottom_left_value(TreeNode::from_vec(vec![2, 1, 3])), 1);
        assert_eq!(
            Solution::find_bottom_left_value(TreeNode::from_vec(vec![1, 2, 3, 4, null, 5, 6, null, null, 7])),
            7
        );
    }
}
