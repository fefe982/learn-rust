// https://leetcode.cn/problems/maximum-depth-of-binary-tree/
// 104. Maximum Depth of Binary Tree
pub struct Solution;
use super::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            Some(node) => {
                let mut node = node.borrow_mut();
                1 + Self::max_depth(node.left.take()).max(Self::max_depth(node.right.take()))
            }
            None => 0,
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_max_depth() {
        let null = super::super::binary_tree::NULL;
        assert_eq!(
            Solution::max_depth(TreeNode::from_vec(vec![3, 9, 20, null, null, 15, 7])),
            3
        );
        assert_eq!(Solution::max_depth(TreeNode::from_vec(vec![1, null, 2])), 2);
    }
}
