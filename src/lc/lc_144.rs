// https://leetcode.com/problems/binary-tree-preorder-traversal/
// 144. Binary Tree Preorder Traversal
pub struct Solution;
use super::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    fn preorder(root: Option<Rc<RefCell<TreeNode>>>, mut v: Vec<i32>) -> Vec<i32> {
        if let Some(node) = root {
            v.push(node.borrow().val);
            v = Self::preorder(node.borrow_mut().left.take(), v);
            v = Self::preorder(node.borrow_mut().right.take(), v);
        }
        v
    }
    pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        Self::preorder(root, vec![])
    }
}
#[cfg(test)]
mod tests {
    use super::super::binary_tree::NULL;
    use super::*;
    #[test]
    fn test_preorder_traversal() {
        let null = NULL;
        assert_eq!(
            Solution::preorder_traversal(TreeNode::from_vec(vec![1, null, 2, 3])),
            vec![1, 2, 3]
        );
        assert_eq!(Solution::preorder_traversal(TreeNode::from_vec(vec![])), vec![]);
        assert_eq!(Solution::preorder_traversal(TreeNode::from_vec(vec![1])), vec![1]);
    }
}
