// https://leetcode.com/problems/construct-binary-tree-from-inorder-and-postorder-traversal/
// 106. Construct Binary Tree from Inorder and Postorder Traversal
pub struct Solution;
use super::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    fn build_tree_slice(inorder: &[i32], postorder: &[i32], root: i32) -> (usize, Option<Rc<RefCell<TreeNode>>>) {
        let mut sz = 0;
        let mut tree = None;
        if inorder.len() > 0 && postorder[0] != root {
            if inorder[0] == postorder[0] {
                tree = Some(Rc::new(RefCell::new(TreeNode::new(postorder[0]))));
                sz = 1;
            }
            while inorder.len() >= sz + 1 && postorder[sz] != root {
                let (rsz, right) = Self::build_tree_slice(&inorder[sz + 1..], &postorder[sz..], inorder[sz]);
                tree = Some(Rc::new(RefCell::new(TreeNode {
                    val: inorder[sz],
                    left: tree,
                    right,
                })));
                sz += rsz + 1;
            }
        }
        (sz, tree)
    }
    pub fn build_tree(inorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::build_tree_slice(&inorder[..], &postorder[..], i32::MIN).1
    }
}
#[cfg(test)]
mod tests {
    use super::super::binary_tree::NULL;
    use super::*;
    #[test]
    fn sorted_list_to_bst() {
        let null = NULL;
        assert_eq!(
            Solution::build_tree(vec![1, 2], vec![2, 1]),
            TreeNode::from_vec(vec![1, null, 2])
        );
        assert_eq!(
            Solution::build_tree(vec![2, 1], vec![2, 1]),
            TreeNode::from_vec(vec![1, 2])
        );
        assert_eq!(
            Solution::build_tree(vec![9, 3, 15, 20, 7], vec![9, 15, 7, 20, 3]),
            TreeNode::from_vec(vec![3, 9, 20, null, null, 15, 7])
        );
        assert_eq!(Solution::build_tree(vec![1], vec![1]), TreeNode::from_vec(vec![1]));
    }
}
