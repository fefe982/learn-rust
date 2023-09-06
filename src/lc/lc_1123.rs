// https://leetcode.com/problems/lowest-common-ancestor-of-deepest-leaves/
// 1123. Lowest Common Ancestor of Deepest Leaves
pub struct Solution;
use super::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    fn deep_leaf(
        root: &Option<Rc<RefCell<TreeNode>>>,
        depth: i32,
    ) -> (Option<Rc<RefCell<TreeNode>>>, i32) {
        if root.is_none() {
            return (None, -1);
        }
        let left = Self::deep_leaf(&root.as_ref().unwrap().borrow().left, depth + 1);
        let right = Self::deep_leaf(&root.as_ref().unwrap().borrow().right, depth + 1);
        if left.1 < 0 && right.1 < 0 {
            (root.clone(), depth)
        } else if left.1 == right.1 {
            (root.clone(), left.1)
        } else if left.1 > right.1 {
            left
        } else {
            right
        }
    }
    pub fn lca_deepest_leaves(
        root: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        Self::deep_leaf(&root, 0).0
    }
}

#[cfg(test)]
mod tests {
    use super::super::binary_tree::NULL;
    use super::*;
    #[test]
    fn lca_deepest_leaves() {
        let null = NULL;
        assert_eq!(
            Solution::lca_deepest_leaves(TreeNode::from_vec(vec![
                3, 5, 1, 6, 2, 0, 8, null, null, 7, 4
            ])),
            TreeNode::from_vec(vec![2, 7, 4])
        );
        assert_eq!(
            Solution::lca_deepest_leaves(TreeNode::from_vec(vec![1])),
            TreeNode::from_vec(vec![1])
        );
        assert_eq!(
            Solution::lca_deepest_leaves(TreeNode::from_vec(vec![0, 1, 3, null, 2])),
            TreeNode::from_vec(vec![2])
        );
    }
}
