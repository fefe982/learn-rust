// https://leetcode.com/problems/binary-tree-zigzag-level-order-traversal/
// 103. Binary Tree Zigzag Level Order Traversal
pub struct Solution;
use super::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    fn levelorder(root: Option<Rc<RefCell<TreeNode>>>, level: usize, mut v: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        if let Some(node) = root {
            if v.len() == level {
                v.push(vec![node.borrow().val]);
            } else {
                v[level].push(node.borrow().val);
            }
            v = Self::levelorder(node.borrow_mut().left.take(), level + 1, v);
            v = Self::levelorder(node.borrow_mut().right.take(), level + 1, v);
        }
        v
    }
    pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let v = Self::levelorder(root, 0, vec![]);
        v.into_iter()
            .enumerate()
            .map(|(i, mut x)| {
                if i % 2 != 0 {
                    x.reverse();
                }
                x
            })
            .collect::<Vec<_>>()
    }
}
#[cfg(test)]
mod tests {
    use super::super::binary_tree::NULL;
    use super::*;
    use crate::*;
    #[test]
    fn test_level_order() {
        let null = NULL;
        assert_eq!(
            Solution::zigzag_level_order(TreeNode::from_vec(vec![3, 9, 20, null, null, 15, 7])),
            vec_vec![[3], [20, 9], [15, 7]]
        );
        assert_eq!(Solution::zigzag_level_order(TreeNode::from_vec(vec![1])), vec_vec![[1]]);
        assert_eq!(
            Solution::zigzag_level_order(TreeNode::from_vec(vec![])),
            Vec::<Vec<i32>>::new()
        );
    }
}
