// https://leetcode.com/problems/increasing-order-search-tree/
// 897. Increasing Order Search Tree
pub struct Solution;
use super::super::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn increasing_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        fn inc_bst(
            root: Option<Rc<RefCell<TreeNode>>>,
        ) -> (Option<Rc<RefCell<TreeNode>>>, Option<Rc<RefCell<TreeNode>>>) {
            if let Some(node) = root {
                let (rr, rt) = inc_bst(node.borrow_mut().right.take());
                node.borrow_mut().right = rr;
                let mut tail = rt;
                if tail.is_none() {
                    tail = Some(node.clone());
                }
                let (lr, lt) = inc_bst(node.borrow_mut().left.take());
                if let Some(ltn) = lt {
                    ltn.borrow_mut().right = Some(node);
                    return (lr, tail);
                } else {
                    return (Some(node), tail);
                }
            } else {
                (None, None)
            }
        }
        inc_bst(root).0
    }
}
#[cfg(test)]
mod tests {
    use super::super::super::binary_tree::NULL;
    use super::*;
    #[test]
    fn increasing_bst() {
        let null = NULL;
        assert_eq!(
            Solution::increasing_bst(TreeNode::from_vec(vec![
                5, 3, 6, 2, 4, null, 8, 1, null, null, null, 7, 9
            ])),
            TreeNode::from_vec(vec![
                1, null, 2, null, 3, null, 4, null, 5, null, 6, null, 7, null, 8, null, 9
            ])
        );
        assert_eq!(
            Solution::increasing_bst(TreeNode::from_vec(vec![5, 1, 7])),
            TreeNode::from_vec(vec![1, null, 5, null, 7])
        );
    }
}
