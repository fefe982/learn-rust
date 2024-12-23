// https://leetcode.com/problems/minimum-number-of-operations-to-sort-a-binary-tree-by-level/
// 2471. Minimum Number of Operations to Sort a Binary Tree by Level
pub struct Solution;
use super::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    fn cnt(nums: Vec<i32>) -> i32 {
        let mut nums = nums.into_iter().enumerate().collect::<Vec<_>>();
        nums.sort_by_key(|&(_, x)| x);
        let mut sum = 0;
        for i in 0..nums.len() {
            if nums[i].0 != i && nums[i].1 != 0 {
                let mut ni = nums[i].0;
                while ni != i {
                    nums[ni].1 = 0;
                    ni = nums[ni].0;
                    sum += 1;
                }
            }
        }
        sum
    }
    fn walk(root: Option<Rc<RefCell<TreeNode>>>, level: usize, nums: &mut Vec<Vec<i32>>) {
        if let Some(node) = root {
            let mut node = node.borrow_mut();
            let val = node.val;
            if nums.len() <= level {
                nums.push(vec![]);
            }
            nums[level].push(val);
            Self::walk(node.left.take(), level + 1, nums);
            Self::walk(node.right.take(), level + 1, nums);
        }
    }
    pub fn minimum_operations(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut nums = vec![];
        Self::walk(root, 0, &mut nums);
        nums.into_iter().skip(1).map(Self::cnt).sum()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn minimum_operations() {
        let null = super::super::binary_tree::NULL;
        assert_eq!(
            Solution::minimum_operations(TreeNode::from_vec(vec![
                1, 4, 3, 7, 6, 8, 5, null, null, null, null, 9, null, 10
            ])),
            3
        );
        assert_eq!(
            Solution::minimum_operations(TreeNode::from_vec(vec![1, 3, 2, 7, 6, 5, 4])),
            3
        );
        assert_eq!(
            Solution::minimum_operations(TreeNode::from_vec(vec![1, 2, 3, 4, 5, 6])),
            0
        );
    }
}
