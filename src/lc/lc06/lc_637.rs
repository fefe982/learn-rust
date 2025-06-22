// https://leetcode.com/problems/average-of-levels-in-binary-tree/
// 637. Average of Levels in Binary Tree
pub struct Solution;
use super::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    fn sum(root: &Option<Rc<RefCell<TreeNode>>>, level: usize, sum: &mut Vec<f64>, count: &mut Vec<usize>) {
        if let Some(node) = root {
            if level >= sum.len() {
                sum.push(node.borrow().val as f64);
                count.push(1);
            } else {
                sum[level] += node.borrow().val as f64;
                count[level] += 1;
            }
            Self::sum(&node.borrow().left, level + 1, sum, count);
            Self::sum(&node.borrow().right, level + 1, sum, count);
        }
    }
    pub fn average_of_levels(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<f64> {
        let mut sum = vec![];
        let mut count = vec![];
        Self::sum(&root, 0, &mut sum, &mut count);
        sum.iter().zip(count.iter()).map(|(s, c)| s / *c as f64).collect()
    }
}
#[cfg(test)]
mod tests {
    use super::super::binary_tree::NULL;
    use super::*;
    #[test]
    fn average_of_levels() {
        let null = NULL;
        assert_eq!(
            Solution::average_of_levels(TreeNode::from_vec(vec![3, 9, 20, null, null, 15, 7])),
            vec![3.0, 14.5, 11.0]
        );
        assert_eq!(
            Solution::average_of_levels(TreeNode::from_vec(vec![3, 9, 20, 15, 7])),
            vec![3.0, 14.5, 11.0]
        );
    }
}
