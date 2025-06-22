// https://leetcode.com/problems/binary-tree-tilt/
// 563. Binary Tree Tilt
pub struct Solution;
use super::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    fn tilt(root: Option<Rc<RefCell<TreeNode>>>) -> (i32, i32) {
        if let Some(root) = root {
            let left = root.borrow_mut().left.take();
            let right = root.borrow_mut().right.take();
            let (l, r) = (Solution::tilt(left), Solution::tilt(right));
            (l.0 + r.0 + (l.1 - r.1).abs(), l.1 + r.1 + root.borrow().val)
        } else {
            return (0, 0);
        }
    }
    pub fn find_tilt(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Solution::tilt(root).0
    }
}
#[cfg(test)]
mod tests {
    use super::super::binary_tree::NULL;
    use super::*;
    #[test]
    fn find_tilt() {
        let null = NULL;
        assert_eq!(Solution::find_tilt(TreeNode::from_vec(vec![1, 2, 3])), 1);
        assert_eq!(
            Solution::find_tilt(TreeNode::from_vec(vec![4, 2, 9, 3, 5, null, 7])),
            15
        );
    }
}
