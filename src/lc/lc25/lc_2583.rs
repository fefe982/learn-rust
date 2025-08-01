// https://leetcode.com/problems/kth-largest-sum-in-a-binary-tree/
// 2583. Kth Largest Sum in a Binary Tree
pub struct Solution;
use super::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn kth_largest_level_sum(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i64 {
        let mut q = std::collections::VecDeque::<(Option<Rc<RefCell<TreeNode>>>, i32)>::new();
        q.push_back((root, 1));
        let mut level_sum = -1;
        let mut last_level = 0;
        let mut heap = std::collections::BinaryHeap::<std::cmp::Reverse<i64>>::new();
        while let Some(r) = q.pop_front() {
            if let (Some(n), level) = r {
                if level != last_level {
                    heap.push(std::cmp::Reverse(level_sum));
                    if heap.len() > k as usize {
                        heap.pop();
                    }
                    last_level = level;
                    level_sum = n.borrow().val as i64;
                } else {
                    level_sum += n.borrow().val as i64;
                }
                q.push_back((n.borrow_mut().left.take(), level + 1));
                q.push_back((n.borrow_mut().right.take(), level + 1));
            }
        }
        heap.push(std::cmp::Reverse(level_sum));
        if heap.len() > k as usize {
            heap.pop();
        }
        if heap.len() == k as usize {
            heap.peek().unwrap().0
        } else {
            -1
        }
    }
}
#[cfg(test)]
mod tests {
    use super::super::binary_tree::NULL;
    use super::*;
    #[test]
    fn test_kth_largest_level_sum() {
        let null = NULL;
        assert_eq!(
            Solution::kth_largest_level_sum(TreeNode::from_vec(vec![5, 8, 9, 2, 1, 3, 7, 4, 6]), 2),
            13
        );
        assert_eq!(
            Solution::kth_largest_level_sum(TreeNode::from_vec(vec![1, 2, null, 3]), 1),
            3
        );
    }
}
