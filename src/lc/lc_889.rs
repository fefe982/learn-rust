// https://leetcode.com/problems/construct-binary-tree-from-preorder-and-postorder-traversal/
// 889. Construct Binary Tree from Preorder and Postorder Traversal
pub struct Solution;
use super::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    fn build_sub_tree(preorder: &[i32], postorder: &[i32], root: i32) -> (usize, Option<Rc<RefCell<TreeNode>>>) {
        if postorder.len() == 0 || postorder[0] == root {
            return (0, None);
        }
        let croot = preorder[0];
        let (szleft, left) = Solution::build_sub_tree(&preorder[1..], postorder, croot);
        let (szright, right) = Solution::build_sub_tree(&preorder[1 + szleft..], &postorder[szleft..], croot);
        (
            1 + szleft + szright,
            Some(Rc::new(RefCell::new(TreeNode {
                val: croot,
                left,
                right,
            }))),
        )
    }
    pub fn construct_from_pre_post(preorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::build_sub_tree(&preorder[..], &postorder[..], i32::MIN).1
    }
}
#[cfg(test)]
mod tests {
    use super::super::binary_tree::NULL;
    use super::*;
    #[test]
    fn test_construct_from_pre_post() {
        assert_eq!(
            Solution::construct_from_pre_post(vec![1, 2, 4, 5, 3, 6, 7], vec![4, 5, 2, 6, 7, 3, 1]),
            TreeNode::from_vec(vec![1, 2, 3, 4, 5, 6, 7])
        );
        assert_eq!(
            Solution::construct_from_pre_post(vec![1], vec![1]),
            TreeNode::from_vec(vec![1])
        );
    }
}
