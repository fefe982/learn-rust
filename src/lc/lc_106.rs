// https://leetcode.com/problems/construct-binary-tree-from-inorder-and-postorder-traversal/
// 106. Construct Binary Tree from Inorder and Postorder Traversal
pub struct Solution;
use super::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    fn build_tree_slice(inorder: &[i32], postorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        if postorder.len() == 0 {
            return None;
        }
        let l = postorder.len();
        let val = postorder[l - 1];
        let mut root = TreeNode::new(val);
        let mut in_val_idx = 0;
        for (idx, v) in inorder.iter().enumerate() {
            if *v == val {
                in_val_idx = idx;
                break;
            }
        }
        root.left = Self::build_tree_slice(&inorder[0..in_val_idx], &postorder[0..in_val_idx]);
        root.right = Self::build_tree_slice(
            &inorder[(in_val_idx + 1)..l],
            &postorder[in_val_idx..(l - 1)],
        );
        Some(Rc::new(RefCell::new(root)))
    }
    pub fn build_tree(inorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::build_tree_slice(&inorder[..], &postorder[..])
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sorted_list_to_bst() {
        assert_eq!(
            Solution::build_tree(vec![9, 3, 15, 20, 7], vec![9, 15, 7, 20, 3]),
            TreeNode::from_vec(&vec![3, 9, 20, -1, -1, 15, 7])
        );
        assert_eq!(
            Solution::build_tree(vec![1], vec![1]),
            TreeNode::from_vec(&vec![1])
        );
    }
}
