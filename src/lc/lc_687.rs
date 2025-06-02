// https://leetcode.com/problems/longest-univalue-path/
// 687. Longest Univalue Path
pub struct Solution;
use super::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    fn univalue_path(root: Option<Rc<RefCell<TreeNode>>>, val: i32) -> (i32, i32) {
        if let Some(node) = root {
            let cval = node.borrow().val;
            let (l, r) = (
                Self::univalue_path(node.borrow().left.clone(), cval),
                Self::univalue_path(node.borrow().right.clone(), cval),
            );
            (
                if val == cval { l.0.max(r.0) + 1 } else { 0 },
                (l.1.max(r.1)).max(l.0 + r.0 + 1),
            )
        } else {
            (0, 0)
        }
    }
    pub fn longest_univalue_path(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_some() {
            Self::univalue_path(root, i32::MAX).1 - 1
        } else {
            0
        }
    }
}
#[cfg(test)]
mod tests {
    use super::super::binary_tree::NULL;
    use super::*;
    #[test]
    fn longest_univalue_path() {
        let null = NULL;
        assert_eq!(
            Solution::longest_univalue_path(TreeNode::from_vec(vec![5, 4, 5, 1, 1, null, 5])),
            2
        );
        assert_eq!(
            Solution::longest_univalue_path(TreeNode::from_vec(vec![1, 4, 5, 4, 4, null, 5])),
            2
        );
    }
}
