// https://leetcode.com/problems/most-frequent-subtree-sum/
// 508. Most Frequent Subtree Sum
pub struct Solution;
use super::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    fn walk(root: Option<Rc<RefCell<TreeNode>>>, map: &mut std::collections::HashMap<i32, i32>) -> (i32, i32) {
        if let Some(node) = root {
            let node = node.borrow();
            let (left_freq, left_sum) = Self::walk(node.left.clone(), map);
            let (right_freq, right_sum) = Self::walk(node.right.clone(), map);
            let sum = node.val + left_sum + right_sum;
            let freq = map.entry(sum).or_insert(0);
            *freq += 1;
            ((*freq).max(left_freq).max(right_freq), sum)
        } else {
            (0, 0)
        }
    }
    pub fn find_frequent_tree_sum(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut map = std::collections::HashMap::new();
        let (max_freq, _) = Self::walk(root, &mut map);
        let mut res = Vec::new();
        for (key, value) in map {
            if value == max_freq {
                res.push(key);
            }
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn find_frequent_tree_sum() {
        assert_sort_eq!(
            Solution::find_frequent_tree_sum(TreeNode::from_vec(vec![5, 2, -3])),
            [2, -3, 4]
        );
        assert_sort_eq!(
            Solution::find_frequent_tree_sum(TreeNode::from_vec(vec![5, 2, -5])),
            [2]
        );
    }
}
