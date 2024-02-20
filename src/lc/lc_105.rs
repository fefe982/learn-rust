// https://leetcode.com/problems/construct-binary-tree-from-preorder-and-inorder-traversal/
// 105. Construct Binary Tree from Preorder and Inorder Traversal
pub struct Solution;
use super::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    fn build_sub_tree(preorder: &[i32], inorder: &[i32], root: i32) -> (usize, Option<Rc<RefCell<TreeNode>>>) {
        if inorder.len() == 0 || inorder[0] == root {
            return (0, None);
        }
        let croot = preorder[0];
        let (szleft, left) = Solution::build_sub_tree(&preorder[1..], inorder, croot);
        let (szright, right) = Solution::build_sub_tree(&preorder[1 + szleft..], &inorder[1 + szleft..], root);
        (
            1 + szleft + szright,
            Some(Rc::new(RefCell::new(TreeNode {
                val: croot,
                left,
                right,
            }))),
        )
    }
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::build_sub_tree(&preorder, &inorder, i32::MIN).1
    }
}
#[cfg(test)]
mod tests {
    use super::super::binary_tree::NULL;
    use super::*;
    #[test]
    fn test_build_tree() {
        let null = NULL;
        assert_eq!(
            Solution::build_tree(vec![3, 9, 20, 15, 7], vec![9, 3, 15, 20, 7]),
            TreeNode::from_vec(vec![3, 9, 20, null, null, 15, 7])
        );
        assert_eq!(Solution::build_tree(vec![-1], vec![-1]), TreeNode::from_vec(vec![-1]));
    }
}
