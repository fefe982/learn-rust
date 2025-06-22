// https://leetcode.com/problems/merge-two-binary-trees/
// 617. Merge Two Binary Trees
pub struct Solution;
use super::binary_tree::TreeNode;
use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;
impl Solution {
    pub fn merge_trees(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        match (root1, root2) {
            (None, None) => None,
            (Some(node1), None) => Some(node1),
            (None, Some(node2)) => Some(node2),
            (Some(node1), Some(node2)) => {
                let mut root = TreeNode::new(node1.borrow().val + node2.borrow().val);
                root.left = Self::merge_trees(
                    node1.deref().borrow_mut().left.take(),
                    node2.deref().borrow_mut().left.take(),
                );
                root.right = Self::merge_trees(
                    node1.deref().borrow_mut().right.take(),
                    node2.deref().borrow_mut().right.take(),
                );
                Some(Rc::new(RefCell::new(root)))
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::super::binary_tree::NULL;
    use super::*;
    #[test]
    fn merge_trees() {
        let null = NULL;
        assert_eq!(
            Solution::merge_trees(
                TreeNode::from_vec(vec![1, 3, 2, 5]),
                TreeNode::from_vec(vec![2, 1, 3, null, 4, null, 7])
            ),
            TreeNode::from_vec(vec![3, 4, 5, 5, 4, null, 7])
        );
        assert_eq!(
            Solution::merge_trees(TreeNode::from_vec(vec![1]), TreeNode::from_vec(vec![1, 2])),
            TreeNode::from_vec(vec![2, 2])
        );
    }
}
