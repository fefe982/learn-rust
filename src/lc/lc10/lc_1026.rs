// https://leetcode.com/problems/maximum-difference-between-node-and-ancestor/
// 1026. Maximum Difference Between Node and Ancestor
pub struct Solution;
use super::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    fn max_ancestor_diff_node(root: &Option<Rc<RefCell<TreeNode>>>) -> (i32, i32, i32) {
        if let Some(node) = root {
            let val_l = Self::max_ancestor_diff_node(&node.as_ref().borrow().left);
            let val_r = Self::max_ancestor_diff_node(&node.as_ref().borrow().right);
            let val_min = std::cmp::min(val_l.1, val_r.1);
            let val_max = std::cmp::max(val_l.2, val_r.2);
            let mut max = std::cmp::max(val_l.0, val_r.0);
            let val = node.as_ref().borrow().val;
            if val_min != i32::MAX {
                max = std::cmp::max(max, (val - val_min).abs());
            }
            if val_max != i32::MIN {
                max = std::cmp::max(max, (val - val_max).abs());
            }
            (
                max,
                std::cmp::min(val_min, val),
                std::cmp::max(val_max, val),
            )
        } else {
            (0, i32::MAX, i32::MIN)
        }
    }
    pub fn max_ancestor_diff(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::max_ancestor_diff_node(&root).0
    }
}
#[cfg(test)]
mod tests {
    use super::super::binary_tree::NULL;
    use super::*;
    #[test]
    fn max_ancestor_diff() {
        assert_eq!(
            Solution::max_ancestor_diff(TreeNode::from_vec(vec![
                8, 3, 10, 1, 6, NULL, 14, NULL, NULL, 4, 7, 13
            ])),
            7
        );
        assert_eq!(
            Solution::max_ancestor_diff(TreeNode::from_vec(vec![1, NULL, 2, NULL, 0, 3])),
            3
        );
    }
}
