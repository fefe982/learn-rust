// https://leetcode.com/problems/deepest-leaves-sum/
// 1302. Deepest Leaves Sum
pub struct Solution;
use super::super::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    fn depth_leaves_sum(root: Option<Rc<RefCell<TreeNode>>>, depth: i32, max_depth: &mut i32, sum: &mut i32) {
        if let Some(node) = root {
            let mut node = node.borrow_mut();
            if depth > *max_depth {
                *max_depth = depth;
                *sum = node.val;
            } else if depth == *max_depth {
                *sum += node.val;
            }
            Self::depth_leaves_sum(node.left.take(), depth + 1, max_depth, sum);
            Self::depth_leaves_sum(node.right.take(), depth + 1, max_depth, sum);
        }
    }
    pub fn deepest_leaves_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut max_depth = 0;
        let mut sum = 0;
        Self::depth_leaves_sum(root, 0, &mut max_depth, &mut sum);
        sum
    }
}
#[cfg(test)]
mod tests {
    use super::super::super::binary_tree::NULL;
    use super::*;
    #[test]
    fn deepest_leaves_sum() {
        let null = NULL;
        assert_eq!(
            Solution::deepest_leaves_sum(TreeNode::from_vec(vec![
                1, 2, 3, 4, 5, null, 6, 7, null, null, null, null, 8
            ])),
            15
        );
        assert_eq!(
            Solution::deepest_leaves_sum(TreeNode::from_vec(vec![
                6, 7, 8, 2, 7, 1, 3, 9, null, 1, 4, null, null, null, 5
            ])),
            19
        );
    }
}
