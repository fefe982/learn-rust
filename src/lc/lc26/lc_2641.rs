// https://leetcode.com/problems/cousins-in-binary-tree-ii/
// 2641. Cousins in Binary Tree II
pub struct Solution;
use super::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    fn sum_tree(root: &Option<Rc<RefCell<TreeNode>>>, depth: usize, sum: &mut Vec<i32>) {
        if let Some(node) = root {
            if depth == sum.len() {
                sum.push(node.borrow().val);
            } else {
                sum[depth] += node.borrow().val;
            }
            Self::sum_tree(&node.borrow().left, depth + 1, sum);
            Self::sum_tree(&node.borrow().right, depth + 1, sum);
        }
    }
    fn mod_tree(
        left: &Option<Rc<RefCell<TreeNode>>>,
        right: &Option<Rc<RefCell<TreeNode>>>,
        depth: usize,
        sum: &Vec<i32>,
    ) {
        if depth == sum.len() {
            return;
        }
        let mut s = 0;
        if let Some(node) = left {
            s += node.borrow().val;
        }
        if let Some(node) = right {
            s += node.borrow().val;
        }
        s = sum[depth] - s;
        if let Some(node) = left {
            node.borrow_mut().val = s;
            Self::mod_tree(&node.borrow().left, &node.borrow().right, depth + 1, sum)
        }
        if let Some(node) = right {
            node.borrow_mut().val = s;
            Self::mod_tree(&node.borrow().left, &node.borrow().right, depth + 1, sum)
        }
    }
    pub fn replace_value_in_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut sum = vec![];
        Self::sum_tree(&root, 0, &mut sum);
        Self::mod_tree(&root, &None, 0, &sum);
        root
    }
}
#[cfg(test)]
mod tests {
    use super::super::binary_tree::NULL;
    use super::*;
    #[test]
    fn test_replace_value_in_tree() {
        let null = NULL;
        assert_eq!(
            Solution::replace_value_in_tree(TreeNode::from_vec(vec![5, 4, 9, 1, 10, null, 7])),
            TreeNode::from_vec(vec![0, 0, 0, 7, 7, null, 11])
        );
        assert_eq!(
            Solution::replace_value_in_tree(TreeNode::from_vec(vec![3, 1, 2])),
            TreeNode::from_vec(vec![0, 0, 0])
        );
    }
}
