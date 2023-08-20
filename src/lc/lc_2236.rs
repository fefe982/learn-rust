// https://leetcode.com/problems/root-equals-sum-of-children/
// 2236. Root Equals Sum of Children
pub struct Solution;
use super::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn check_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let rn = root.unwrap();
        let r = rn.borrow().val;
        let rl = rn.borrow().left.as_ref().unwrap().borrow().val;
        let rr = rn.borrow().right.as_ref().unwrap().borrow().val;
        r == rl + rr
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn check_tree() {
        assert_eq!(
            Solution::check_tree(TreeNode::from_vec(vec![5, 3, 1])),
            false
        );
        assert_eq!(
            Solution::check_tree(TreeNode::from_vec(vec![10, 4, 6])),
            true
        );
    }
}
