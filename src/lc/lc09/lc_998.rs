// https://leetcode.com/problems/maximum-binary-tree-ii/
// 998. Maximum Binary Tree II
pub struct Solution;
use super::super::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn insert_into_max_tree(root: Option<Rc<RefCell<TreeNode>>>, val: i32) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(r) = root {
            if r.borrow().val < val {
                let mut node = TreeNode::new(val);
                node.left = Some(r);
                Some(Rc::new(RefCell::new(node)))
            } else {
                let right = Self::insert_into_max_tree(r.borrow_mut().right.take(), val);
                r.borrow_mut().right = right;
                Some(r)
            }
        } else {
            Some(Rc::new(RefCell::new(TreeNode::new(val))))
        }
    }
}
#[cfg(test)]
mod tests {
    use super::super::super::binary_tree::NULL;
    use super::*;
    #[test]
    fn insert_into_max_tree() {
        let null = NULL;
        assert_eq!(
            Solution::insert_into_max_tree(TreeNode::from_vec(vec![4, 1, 3, null, null, 2]), 5),
            TreeNode::from_vec(vec![5, 4, null, 1, 3, null, null, 2])
        );
        assert_eq!(
            Solution::insert_into_max_tree(TreeNode::from_vec(vec![5, 2, 4, null, 1]), 3),
            TreeNode::from_vec(vec![5, 2, 4, null, 1, null, 3])
        );
        assert_eq!(
            Solution::insert_into_max_tree(TreeNode::from_vec(vec![5, 2, 3, null, 1]), 4),
            TreeNode::from_vec(vec![5, 2, 4, null, 1, 3])
        );
    }
}
