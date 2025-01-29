// https://leetcode.com/problems/convert-sorted-array-to-binary-search-tree/
// 108. Convert Sorted Array to Binary Search Tree
pub struct Solution;
use super::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    fn make_tree(nums: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        if nums.is_empty() {
            return None;
        }
        let mid = nums.len() / 2;
        Some(Rc::new(RefCell::new(TreeNode {
            val: nums[mid],
            left: Self::make_tree(&nums[0..mid]),
            right: Self::make_tree(&nums[mid + 1..]),
        })))
    }
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::make_tree(nums.as_slice())
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    fn check(nums: Vec<i32>) {
        let res = Solution::sorted_array_to_bst(nums.clone());
        assert_eq!(nums, super::super::lc_94::Solution::inorder_traversal(res.clone()));
        assert!(super::super::lc_110::Solution::is_balanced(res.clone()));
        assert!(super::super::lc_98::Solution::is_valid_bst(res));
    }
    #[test]
    fn test_sorted_array_to_bst() {
        check(vec![-10, -3, 0, 5, 9]);
        check(vec![1, 3]);
    }
}
