// https://leetcode.com/problems/binary-tree-pruning/
// 814. Binary Tree Pruning
pub struct Solution;
use super::super::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn prune_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(root) = root {
            let left = Self::prune_tree(root.borrow_mut().left.take());
            let right = Self::prune_tree(root.borrow_mut().right.take());
            if root.borrow().val == 0 && left.is_none() && right.is_none() {
                return None;
            }
            root.borrow_mut().left = left;
            root.borrow_mut().right = right;
            Some(root)
        } else {
            None
        }
    }
}
#[cfg(test)]
mod tests {
    use super::super::super::binary_tree::NULL;
    use super::*;
    #[test]
    fn prune_tree() {
        let null = NULL;
        assert_eq!(
            Solution::prune_tree(TreeNode::from_vec(vec![1, null, 0, 0, 1])),
            TreeNode::from_vec(vec![1, null, 0, null, 1])
        );
        assert_eq!(
            Solution::prune_tree(TreeNode::from_vec(vec![1, 0, 1, 0, 0, 0, 1])),
            TreeNode::from_vec(vec![1, null, 1, null, 1])
        );
    }
}
