// https://leetcode.com/problems/all-nodes-distance-k-in-binary-tree/
// 863. All Nodes Distance K in Binary Tree
pub struct Solution;
use super::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    fn distance_k_inner(
        root: &Option<Rc<RefCell<TreeNode>>>,
        target: i32,
        k: i32,
        found: bool,
        v: &mut Vec<i32>,
    ) -> i32 {
        if let Some(node) = root {
            let node = node.borrow();
            let f = found || (node.val == target);
            if f {
                if k == 0 {
                    v.push(node.val);
                } else if k > 0 {
                    Self::distance_k_inner(&node.left, target, k - 1, true, v);
                    Self::distance_k_inner(&node.right, target, k - 1, true, v);
                }
                0
            } else {
                let dl = Self::distance_k_inner(&node.left, target, k, false, v);
                let dr = Self::distance_k_inner(&node.right, target, k, false, v);
                if dl == i32::MAX && dr == i32::MAX {
                    i32::MAX
                } else {
                    let d = if dl != i32::MAX { dl } else { dr } + 1;
                    let n = if dl != i32::MAX {
                        &node.right
                    } else {
                        &node.left
                    };
                    if d == k {
                        v.push(node.val);
                    } else if d < k {
                        Self::distance_k_inner(n, target, k - d - 1, true, v);
                    }
                    d
                }
            }
        } else {
            i32::MAX
        }
    }
    pub fn distance_k(
        root: Option<Rc<RefCell<TreeNode>>>,
        target: Option<Rc<RefCell<TreeNode>>>,
        k: i32,
    ) -> Vec<i32> {
        let mut v = vec![];
        Self::distance_k_inner(&root, target.unwrap().borrow().val, k, false, &mut v);
        v
    }
}
#[cfg(test)]
mod tests {
    use std::vec;

    use super::super::binary_tree::NULL;
    use super::*;
    #[test]
    fn distance_k() {
        let null = NULL;
        assert_eq!(
            Solution::distance_k(
                TreeNode::from_vec(vec![3, 5, 1, 6, 2, 0, 8, null, null, 7, 4]),
                TreeNode::from_vec(vec![5]),
                2
            ),
            vec![7, 4, 1]
        );
        assert_eq!(
            Solution::distance_k(TreeNode::from_vec(vec![1]), TreeNode::from_vec(vec![1]), 3),
            vec![]
        );
    }
}
