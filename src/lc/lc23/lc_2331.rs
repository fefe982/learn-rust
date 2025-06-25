// https://leetcode.com/problems/evaluate-boolean-binary-tree/
// 2331. Evaluate Boolean Binary Tree
pub struct Solution;
use super::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn evaluate_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let root = root.unwrap();
        let node = root.borrow();
        match node.val {
            0 => false,
            1 => true,
            2 => Self::evaluate_tree(node.left.clone()) || Self::evaluate_tree(node.right.clone()),
            3 => Self::evaluate_tree(node.left.clone()) && Self::evaluate_tree(node.right.clone()),
            _ => unreachable!(),
        }
    }
}
#[cfg(test)]
mod tests {
    use super::super::binary_tree::NULL;
    use super::*;
    #[test]
    fn test_evaluate_tree() {
        let null = NULL;
        assert_eq!(
            Solution::evaluate_tree(TreeNode::from_vec(vec![
                3, 3, 2, 0, 0, 3, 2, null, null, null, null, 1, 3, 1, 1, null, null, 2, 1, null, null, null, null, 1,
                1, null, null, null, null, null, null
            ])),
            false
        );
        assert_eq!(
            Solution::evaluate_tree(TreeNode::from_vec(vec![2, 1, 3, null, null, 0, 1])),
            true
        );
        assert_eq!(Solution::evaluate_tree(TreeNode::from_vec(vec![0])), false);
    }
}
