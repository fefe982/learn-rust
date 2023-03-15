// https://leetcode.com/problems/check-completeness-of-a-binary-tree/
// 958. Check Completeness of a Binary Tree
pub struct Solution;
use super::binary_tree::TreeNode;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
impl Solution {
    pub fn is_complete_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if let Some(node) = root {
            let mut q = VecDeque::new();
            q.push_back(node);
            let mut finish = false;
            while q.len() > 0 {
                let node = q.pop_front().unwrap();
                if let Some(n) = node.borrow_mut().left.take() {
                    if finish {
                        return false;
                    } else {
                        q.push_back(n);
                    }
                } else {
                    finish = true;
                };
                if let Some(n) = node.borrow_mut().right.take() {
                    if finish {
                        return false;
                    } else {
                        q.push_back(n);
                    }
                } else {
                    finish = true;
                };
            }
            true
        } else {
            true
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn is_complete_tree() {
        assert_eq!(
            Solution::is_complete_tree(TreeNode::from_vec(&vec![1, 2, 3, 4, 5, 6])),
            true
        );
        assert_eq!(
            Solution::is_complete_tree(TreeNode::from_vec(&vec![1, 2, 3, 4, 5, -1, 7])),
            false
        );
    }
}
