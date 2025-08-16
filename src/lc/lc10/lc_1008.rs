// https://leetcode.com/problems/construct-binary-search-tree-from-preorder-traversal/
// 1008. Construct Binary Search Tree from Preorder Traversal
pub struct Solution;
use super::super::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    fn bst_preorder(preorder: &[i32], high: i32) -> (Option<Rc<RefCell<TreeNode>>>, usize) {
        if preorder.is_empty() || preorder[0] > high {
            return (None, 0);
        }
        let mut node = TreeNode::new(preorder[0]);
        let (left, lenl) = Self::bst_preorder(&preorder[1..], preorder[0]);
        node.left = left;
        let (right, lenr) = Self::bst_preorder(&preorder[1 + lenl..], high);
        node.right = right;
        (Some(Rc::new(RefCell::new(node))), lenl + lenr + 1)
    }
    pub fn bst_from_preorder(preorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let (root, _) = Self::bst_preorder(&preorder, i32::MAX);
        root
    }
}
#[cfg(test)]
mod tests {
    use super::super::super::binary_tree::NULL;
    use super::*;
    #[test]
    fn bst_from_preorder() {
        let null = NULL;
        assert_eq!(
            Solution::bst_from_preorder(vec![8, 5, 1, 7, 10, 12]),
            TreeNode::from_vec(vec![8, 5, 10, 1, 7, null, 12])
        );
        assert_eq!(
            Solution::bst_from_preorder(vec![1, 3]),
            TreeNode::from_vec(vec![1, null, 3])
        );
    }
}
