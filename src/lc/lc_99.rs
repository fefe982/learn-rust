// https://leetcode.com/problems/recover-binary-search-tree/
// 99. Recover Binary Search Tree
pub struct Solution;
use super::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    fn walk(root: &mut Option<Rc<RefCell<TreeNode>>>, prev: &mut i32, recover: bool, swap: &mut Vec<i32>) -> bool {
        if let Some(node) = root {
            let mut node = node.borrow_mut();
            if !Self::walk(&mut node.left, prev, recover, swap) {
                return false;
            }
            if recover {
                if node.val == swap[0] {
                    node.val = swap[1];
                } else if node.val == swap[1] {
                    node.val = swap[0];
                    return false;
                }
            } else {
                if node.val < *prev {
                    if swap.is_empty() {
                        swap.push(*prev);
                        swap.push(node.val);
                    } else {
                        swap[1] = node.val;
                        return false;
                    }
                }
                *prev = node.val;
            }
            if !Self::walk(&mut node.right, prev, recover, swap) {
                return false;
            }
        }
        true
    }
    pub fn recover_tree(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        let mut swap = vec![];
        let mut prev = i32::MIN;
        Self::walk(root, &mut prev, false, &mut swap);
        Self::walk(root, &mut prev, true, &mut swap);
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    fn check(mut root: Option<Rc<RefCell<TreeNode>>>, expected: Option<Rc<RefCell<TreeNode>>>) {
        Solution::recover_tree(&mut root);
        assert_eq!(root, expected);
    }
    #[test]
    fn test_recover_tree() {
        let null = super::super::binary_tree::NULL;
        check(
            TreeNode::from_vec(vec![1, 3, null, null, 2]),
            TreeNode::from_vec(vec![3, 1, null, null, 2]),
        );
        check(
            TreeNode::from_vec(vec![3, 1, 4, null, null, 2]),
            TreeNode::from_vec(vec![2, 1, 4, null, null, 3]),
        );
    }
}
