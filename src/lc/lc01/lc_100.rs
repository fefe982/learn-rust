// https://leetcode.com/problems/same-tree/
// 100. Same Tree
pub struct Solution;
use super::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match (p, q) {
            (None, None) => true,
            (Some(p), Some(q)) => {
                let p = p.borrow();
                let q = q.borrow();
                p.val == q.val
                    && Self::is_same_tree(p.left.clone(), q.left.clone())
                    && Self::is_same_tree(p.right.clone(), q.right.clone())
            }
            _ => false,
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_is_same_tree() {
        let null = super::super::binary_tree::NULL;
        assert_eq!(
            Solution::is_same_tree(TreeNode::from_vec(vec![1, 2, 3]), TreeNode::from_vec(vec![1, 2, 3])),
            true
        );
        assert_eq!(
            Solution::is_same_tree(TreeNode::from_vec(vec![1, 2]), TreeNode::from_vec(vec![1, null, 2])),
            false
        );
        assert_eq!(
            Solution::is_same_tree(TreeNode::from_vec(vec![1, 2, 1]), TreeNode::from_vec(vec![1, 1, 2])),
            false
        );
    }
}
