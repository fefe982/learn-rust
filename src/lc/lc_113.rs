// https://leetcode.com/problems/path-sum-ii/
// 113. Path Sum II
pub struct Solution;
use super::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    fn walk(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32, pre: &mut Vec<i32>, res: &mut Vec<Vec<i32>>) {
        if let Some(r) = root {
            let mut r = r.borrow_mut();
            pre.push(r.val);
            if r.left.is_none() && r.right.is_none() && r.val == target_sum {
                res.push(pre.clone());
            }
            Self::walk(r.left.take(), target_sum - r.val, pre, res);
            Self::walk(r.right.take(), target_sum - r.val, pre, res);
            pre.pop();
        }
    }
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> Vec<Vec<i32>> {
        let mut res = vec![];
        Self::walk(root, target_sum, &mut vec![], &mut res);
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_path_sum() {
        let null = super::super::binary_tree::NULL;
        assert_sort_eq!(
            Solution::path_sum(
                TreeNode::from_vec(vec![5, 4, 8, 11, null, 13, 4, 7, 2, null, null, 5, 1]),
                22
            ),
            vec_vec![[5, 4, 11, 2], [5, 8, 4, 5]]
        );
        assert_sort_eq!(
            Solution::path_sum(TreeNode::from_vec(vec![1, 2, 3]), 5),
            Vec::<Vec<i32>>::new()
        );
        assert_sort_eq!(
            Solution::path_sum(TreeNode::from_vec(vec![1, 2]), 0),
            Vec::<Vec<i32>>::new()
        );
    }
}
