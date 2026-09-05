// https://leetcode.com/problems/k-th-largest-perfect-subtree-size-in-binary-tree/
// 3319. K-th Largest Perfect Subtree Size in Binary Tree
use super::super::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
pub struct Solution;
impl Solution {
    fn wark(
        root: &Option<Rc<RefCell<TreeNode>>>,
        k: usize,
        h: &mut std::collections::BinaryHeap<std::cmp::Reverse<i32>>,
    ) -> i32 {
        if let Some(node) = root {
            let l = Self::wark(&node.borrow().left, k, h);
            let r = Self::wark(&node.borrow().right, k, h);
            if l >= 0 && l == r {
                h.push(std::cmp::Reverse(l + r + 1));
                while h.len() > k {
                    h.pop();
                }
                l + r + 1
            } else {
                -1
            }
        } else {
            0
        }
    }
    pub fn kth_largest_perfect_subtree(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        let mut h = std::collections::BinaryHeap::new();
        let k = k as usize;
        Self::wark(&root, k, &mut h);
        if h.len() < k {
            -1
        } else {
            h.pop().unwrap().0
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn kth_largest_perfect_subtree() {
        let null = super::super::super::binary_tree::NULL;
        assert_eq!(
            Solution::kth_largest_perfect_subtree(TreeNode::from_vec(vec![1, 11, 13]), 2),
            1
        );
        assert_eq!(
            Solution::kth_largest_perfect_subtree(
                TreeNode::from_vec(vec![5, 3, 6, 5, 2, 5, 7, 1, 8, null, null, 6, 8]),
                2
            ),
            3
        );
        assert_eq!(
            Solution::kth_largest_perfect_subtree(TreeNode::from_vec(vec![1, 2, 3, 4, 5, 6, 7]), 1),
            7
        );
        assert_eq!(
            Solution::kth_largest_perfect_subtree(TreeNode::from_vec(vec![1, 2, 3, null, 4]), 3),
            -1
        );
    }
}
