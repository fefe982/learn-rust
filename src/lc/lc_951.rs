// https://leetcode.com/problems/flip-equivalent-binary-trees/
// 951. Flip Equivalent Binary Trees
pub struct Solution;
use super::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    fn get_children(root: Rc<RefCell<TreeNode>>) -> (Option<Rc<RefCell<TreeNode>>>, Option<Rc<RefCell<TreeNode>>>) {
        let mut rt = root.borrow_mut();
        match (rt.left.take(), rt.right.take()) {
            (Some(n1), Some(n2)) => {
                if n1.borrow().val < n2.borrow().val {
                    (Some(n1), Some(n2))
                } else {
                    (Some(n2), Some(n1))
                }
            }
            (None, Some(n2)) => (Some(n2), None),
            (Some(n1), None) => (Some(n1), None),
            _ => (None, None),
        }
    }
    pub fn flip_equiv(root1: Option<Rc<RefCell<TreeNode>>>, root2: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match (root1, root2) {
            (Some(n1), Some(n2)) => {
                if n1.borrow().val != n2.borrow().val {
                    false
                } else {
                    let (l1, r1) = Self::get_children(n1);
                    let (l2, r2) = Self::get_children(n2);
                    Self::flip_equiv(l1, l2) && Self::flip_equiv(r1, r2)
                }
            }
            (None, None) => true,
            _ => false,
        }
    }
}
#[cfg(test)]
mod tests {
    use super::super::binary_tree::NULL;
    use super::*;
    #[test]
    fn test_flip_equiv() {
        let null = NULL;
        assert_eq!(
            Solution::flip_equiv(
                TreeNode::from_vec(vec![1, 2, 3, 4, 5, 6, null, null, null, 7, 8]),
                TreeNode::from_vec(vec![1, 3, 2, null, 6, 4, 5, null, null, null, null, 8, 7])
            ),
            true
        );
        assert_eq!(
            Solution::flip_equiv(TreeNode::from_vec(vec![]), TreeNode::from_vec(vec![])),
            true
        );
        assert_eq!(
            Solution::flip_equiv(TreeNode::from_vec(vec![]), TreeNode::from_vec(vec![1])),
            false
        );
    }
}
