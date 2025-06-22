// https://leetcode.com/problems/binary-tree-postorder-traversal/
// 145. Binary Tree Postorder Traversal
pub struct Solution;
use super::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    fn postorder(root: Option<Rc<RefCell<TreeNode>>>, mut v: Vec<i32>) -> Vec<i32> {
        if let Some(node) = root {
            v = Self::postorder(node.borrow_mut().left.take(), v);
            v = Self::postorder(node.borrow_mut().right.take(), v);
            v.push(node.borrow().val);
        }
        v
    }
    pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        Self::postorder(root, Vec::new())
    }
}
#[cfg(test)]
mod tests {
    use super::super::binary_tree::NULL;
    use super::*;
    #[test]
    fn test_postorder_traversal() {
        let null = NULL;
        assert_eq!(
            Solution::postorder_traversal(TreeNode::from_vec(vec![1, null, 2, 3])),
            vec![3, 2, 1]
        );
        assert_eq!(Solution::postorder_traversal(TreeNode::from_vec(vec![])), vec![]);
        assert_eq!(Solution::postorder_traversal(TreeNode::from_vec(vec![1])), vec![1]);
    }
}
