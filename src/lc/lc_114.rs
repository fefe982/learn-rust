// https://leetcode.com/problems/flatten-binary-tree-to-linked-list/
// 114. Flatten Binary Tree to Linked List
pub struct Solution;
use super::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    fn fln(root: &mut Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(node) = root {
            let mut nodemut = node.borrow_mut();
            let mut lroot = nodemut.left.take();
            let mut rroot = nodemut.right.take();
            let left = Self::fln(&mut lroot);
            let right = Self::fln(&mut rroot);
            match (lroot, rroot) {
                (Some(l), Some(r)) => {
                    nodemut.right = Some(l);
                    left.unwrap().borrow_mut().right = Some(r);
                    right
                }
                (Some(l), None) => {
                    nodemut.right = Some(l);
                    left
                }
                (None, Some(r)) => {
                    nodemut.right = Some(r);
                    right
                }
                (None, None) => Some(node.clone()),
            }
        } else {
            None
        }
    }
    pub fn flatten(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        Self::fln(root);
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    fn check(root: &mut Option<Rc<RefCell<TreeNode>>>, expect: Option<Rc<RefCell<TreeNode>>>) {
        Solution::flatten(root);
        assert_eq!(root, &expect);
    }
    #[test]
    fn test_flatten() {
        let null = super::super::binary_tree::NULL;
        check(
            &mut TreeNode::from_vec(vec![1, 2, 5, 3, 4, null, 6]),
            TreeNode::from_vec(vec![1, null, 2, null, 3, null, 4, null, 5, null, 6]),
        );
        check(&mut TreeNode::from_vec(vec![]), TreeNode::from_vec(vec![]));
        check(&mut TreeNode::from_vec(vec![0]), TreeNode::from_vec(vec![0]));
    }
}
