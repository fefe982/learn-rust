// https://leetcode.com/problems/sum-of-root-to-leaf-binary-numbers/
// 1022. Sum of Root to Leaf Binary Numbers
pub struct Solution;
use super::super::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn sum_root_to_leaf(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn sum(root: &Option<Rc<RefCell<TreeNode>>>, s: i32) -> i32 {
            if let Some(node) = root {
                let val = s * 2 + node.borrow().val;
                if node.borrow().left.is_none() && node.borrow().right.is_none() {
                    val
                } else {
                    sum(&node.borrow().left, val) + sum(&node.borrow().right, val)
                }
            } else {
                0
            }
        }
        sum(&root, 0)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sum_root_to_leaf() {
        assert_eq!(
            Solution::sum_root_to_leaf(TreeNode::from_vec(vec![1, 0, 1, 0, 1, 0, 1])),
            22
        );
        assert_eq!(Solution::sum_root_to_leaf(TreeNode::from_vec(vec![0])), 0);
    }
}
