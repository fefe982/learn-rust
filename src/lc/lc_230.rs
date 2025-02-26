// https://leetcode.com/problems/kth-smallest-element-in-a-bst/
// 230. Kth Smallest Element in a BST
pub struct Solution;
use super::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    fn search(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> (i32, i32) {
        if let Some(r) = root {
            let (cnt, val) = Self::search(r.borrow_mut().left.take(), k);
            if val >= 0 {
                return (0, val);
            }
            if cnt + 1 == k {
                return (0, r.borrow().val);
            }
            let (rcnt, val) = Self::search(r.borrow_mut().right.take(), k - cnt - 1);
            if val >= 0 {
                (0, val)
            } else {
                (cnt + 1 + rcnt, -1)
            }
        } else {
            (0, -1)
        }
    }
    pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        Self::search(root, k).1
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_kth_smallest() {
        let null = super::super::binary_tree::NULL;
        assert_eq!(Solution::kth_smallest(TreeNode::from_vec(vec![3, 1, 4, null, 2]), 1), 1);
        assert_eq!(
            Solution::kth_smallest(TreeNode::from_vec(vec![5, 3, 6, 2, 4, null, null, 1]), 3),
            3
        );
    }
}
