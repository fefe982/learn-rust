// https://leetcode.com/problems/delete-leaves-with-a-given-value/
// 1325. Delete Leaves With a Given Value
pub struct Solution;
use super::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn remove_leaf_nodes(root: Option<Rc<RefCell<TreeNode>>>, target: i32) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(node) = root {
            {
                let mut node_ref = node.borrow_mut();
                node_ref.left = Self::remove_leaf_nodes(node_ref.left.take(), target);
                node_ref.right = Self::remove_leaf_nodes(node_ref.right.take(), target);
                if node_ref.left.is_none() && node_ref.right.is_none() && node_ref.val == target {
                    return None;
                }
            }
            Some(node)
        } else {
            None
        }
    }
}
#[cfg(test)]
mod tests {
    use super::super::binary_tree::NULL;
    use super::*;
    #[test]
    fn test_remove_leaf_nodes() {
        let null = NULL;
        assert_eq!(
            Solution::remove_leaf_nodes(TreeNode::from_vec(vec![1, 2, 3, 2, null, 2, 4]), 2),
            TreeNode::from_vec(vec![1, null, 3, null, 4])
        );
        assert_eq!(
            Solution::remove_leaf_nodes(TreeNode::from_vec(vec![1, 2, null, 2, null, 2]), 2),
            TreeNode::from_vec(vec![1])
        );
    }
}
