// https://leetcode.com/problems/leaf-similar-trees/
// 872. Leaf-Similar Trees
pub struct Solution;
use super::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    fn get_leaf(root: &Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        if let Some(node) = root {
            let mut left = Solution::get_leaf(&node.borrow().left);
            let mut right = Solution::get_leaf(&node.borrow().right);
            if left.is_empty() && right.is_empty() {
                return vec![node.borrow().val];
            }
            left.append(&mut right);
            return left;
        }
        return vec![];
    }
    pub fn leaf_similar(root1: Option<Rc<RefCell<TreeNode>>>, root2: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::get_leaf(&root1) == Self::get_leaf(&root2)
    }
}
#[cfg(test)]
mod tests {
    use super::super::binary_tree::NULL;
    use super::*;
    #[test]
    fn test_leaf_similar() {
        let null = NULL;
        assert_eq!(
            Solution::leaf_similar(
                TreeNode::from_vec(vec![3, 5, 1, 6, 2, 9, 8, null, null, 7, 4]),
                TreeNode::from_vec(vec![3, 5, 1, 6, 7, 4, 2, null, null, null, null, null, null, 9, 8])
            ),
            true
        );
        assert_eq!(
            Solution::leaf_similar(TreeNode::from_vec(vec![1, 2, 3]), TreeNode::from_vec(vec![1, 3, 2])),
            false
        );
    }
}
