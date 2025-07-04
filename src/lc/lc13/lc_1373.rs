// https://leetcode.com/problems/maximum-sum-bst-in-binary-tree/
// 1373. Maximum Sum BST in Binary Tree
use super::binary_tree::TreeNode;
pub struct Solution;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    fn max_sum_bst_range(root: Rc<RefCell<TreeNode>>) -> (i32, i32, i32, i32) {
        let root = root.borrow();
        let mut max = root.val;
        let mut min = root.val;
        let mut l = 0;
        let mut r = 0;
        let mut val = root.val;
        if let Some(left) = root.left.clone() {
            let (lval, lm, lmin, lmax) = Self::max_sum_bst_range(left);
            l = lm;
            if lmax < min {
                min = lmin;
                val += lval;
            } else {
                min = i32::MIN;
                max = i32::MAX;
            }
        }
        if let Some(right) = root.right.clone() {
            let (rval, rm, rmin, rmax) = Self::max_sum_bst_range(right);
            r = rm;
            if max < rmin {
                max = rmax;
                val += rval;
            } else {
                min = i32::MIN;
                max = i32::MAX;
            }
        }
        let mlr = std::cmp::max(l, r);
        if min == i32::MIN {
            val = mlr;
        }
        return (val, std::cmp::max(val, mlr), min, max);
    }
    pub fn max_sum_bst(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(r) = root {
            Self::max_sum_bst_range(r).1
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
    fn max_sum_bst() {
        assert_eq!(
            Solution::max_sum_bst(TreeNode::from_vec(vec![4, -3, 6, -5, -2, 5, 9])),
            20
        );
        assert_eq!(
            Solution::max_sum_bst(TreeNode::from_vec(vec![
                4, 8, NULL, 6, 1, 9, NULL, -5, 4, NULL, NULL, NULL, -3, NULL, 10
            ])),
            14
        );
        assert_eq!(
            Solution::max_sum_bst(TreeNode::from_vec(vec![1, NULL, 10, -5, 20])),
            25
        );
        assert_eq!(
            Solution::max_sum_bst(TreeNode::from_vec(vec![
                1, 4, 3, 2, 4, 2, 5, NULL, NULL, NULL, NULL, NULL, NULL, 4, 6
            ])),
            20
        );
        assert_eq!(
            Solution::max_sum_bst(TreeNode::from_vec(vec![4, 3, NULL, 1, 2])),
            2
        );
        assert_eq!(
            Solution::max_sum_bst(TreeNode::from_vec(vec![-4, -2, -5])),
            0
        );
    }
}
