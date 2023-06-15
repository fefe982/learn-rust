// https://leetcode.com/problems/maximum-level-sum-of-a-binary-tree/
// 1161. Maximum Level Sum of a Binary Tree
pub struct Solution;
use super::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn max_level_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut q = std::collections::VecDeque::<(Option<Rc<RefCell<TreeNode>>>, i32)>::new();
        q.push_back((root, 1));
        let mut level_sum = i32::MIN;
        let mut last_level = 0;
        let mut max = i32::MIN;
        let mut max_level = 0;
        while let Some(r) = q.pop_front() {
            if let (Some(n), level) = r {
                if level != last_level {
                    if level_sum > max {
                        max = level_sum;
                        max_level = last_level;
                    }
                    last_level = level;
                    level_sum = n.borrow().val;
                } else {
                    level_sum += n.borrow().val;
                }
                q.push_back((n.borrow_mut().left.take(), level + 1));
                q.push_back((n.borrow_mut().right.clone(), level + 1));
            }
        }
        if level_sum > max {
            last_level
        } else {
            max_level
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn max_level_sum() {
        let null = super::super::binary_tree::NULL;
        assert_eq!(
            Solution::max_level_sum(TreeNode::from_vec(vec![1, 7, 0, 7, -8, null, null])),
            2
        );
        assert_eq!(
            Solution::max_level_sum(TreeNode::from_vec(vec![
                989, null, 10250, 98693, -89388, null, null, null, -32127
            ])),
            2
        );
    }
}
