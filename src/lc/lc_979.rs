// https://leetcode.com/problems/distribute-coins-in-binary-tree/
// 979. Distribute Coins in Binary Tree
pub struct Solution;
use super::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    fn dist(root: &Option<Rc<RefCell<TreeNode>>>) -> (i32, i32) {
        match root {
            &None => (0, 0),
            &Some(ref node) => {
                let node = node.borrow();
                let (vl, sl) = Self::dist(&node.left);
                let (vr, sr) = Self::dist(&node.right);
                (vl + vr + node.val - 1, sl + sr + vl.abs() + vr.abs())
            }
        }
    }
    pub fn distribute_coins(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::dist(&root).1
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn distribute_coins() {
        assert_eq!(
            Solution::distribute_coins(TreeNode::from_vec(vec![3, 0, 0])),
            2
        );
        assert_eq!(
            Solution::distribute_coins(TreeNode::from_vec(vec![0, 3, 0])),
            3
        );
    }
}
