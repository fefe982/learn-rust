// https://leetcode.com/problems/second-minimum-node-in-a-binary-tree/
// 671. Second Minimum Node In a Binary Tree
pub struct Solution;
use super::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    fn find(root: Option<Rc<RefCell<TreeNode>>>, min: i32, second_min: &mut i32) {
        if let Some(node) = root {
            let mut node = node.borrow_mut();
            if node.val > min {
                if node.val < *second_min || *second_min == -1 {
                    *second_min = node.val;
                }
                return;
            }
            Self::find(node.left.take(), min, second_min);
            Self::find(node.right.take(), min, second_min);
        }
    }
    pub fn find_second_minimum_value(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(node) = root {
            let min = node.borrow().val;
            let mut second_min = -1;
            Self::find(Some(node), min, &mut second_min);
            return second_min;
        }
        -1
    }
}
#[cfg(test)]
mod tests {
    use super::super::binary_tree::NULL;
    use super::*;
    #[test]
    fn find_second_minimum_value() {
        assert_eq!(
            Solution::find_second_minimum_value(TreeNode::from_vec(vec![2, 2, 5, NULL, NULL, 5, 7])),
            5
        );
        assert_eq!(
            Solution::find_second_minimum_value(TreeNode::from_vec(vec![2, 2, 2])),
            -1
        );
    }
}
