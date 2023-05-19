// https://leetcode.com/problems/sum-root-to-leaf-numbers/
// 129. Sum Root to Leaf Numbers
pub struct Solution;
use super::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    fn sum_tree(root: &Option<Rc<RefCell<TreeNode>>>, partial: i32, sum: &mut i32) {
        if let Some(ref node) = root {
            let node = node.as_ref().borrow();
            let partial = partial * 10 + node.val;
            if let None = node.left {
                if let None = node.right {
                    *sum += partial;
                    return;
                }
            }
            Self::sum_tree(&node.left, partial, sum);
            Self::sum_tree(&node.right, partial, sum);
        }
    }
    pub fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut sum = 0;
        Self::sum_tree(&root, 0, &mut sum);
        sum
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sum_numbers() {
        assert_eq!(Solution::sum_numbers(TreeNode::from_vec(vec![1, 2, 3])), 25);
        assert_eq!(
            Solution::sum_numbers(TreeNode::from_vec(vec![4, 9, 0, 5, 1])),
            1026
        );
    }
}
