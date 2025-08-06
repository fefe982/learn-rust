// https://leetcode.com/problems/univalued-binary-tree/
// 965. Univalued Binary Tree
pub struct Solution;
use super::super::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn is_unival_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn unival(root: &Option<Rc<RefCell<TreeNode>>>, val: i32) -> bool {
            if let Some(node) = root {
                let node = node.borrow();
                if val != -1 && node.val != val {
                    return false;
                }
                return unival(&node.left, node.val) && unival(&node.right, node.val);
            }
            true
        }
        unival(&root, -1)
    }
}
#[cfg(test)]
mod tests {
    use crate::lc::binary_tree::TreeNode;

    use super::super::super::binary_tree::NULL;
    use super::*;
    #[test]
    fn is_unival_tree() {
        let null = NULL;
        assert_eq!(
            Solution::is_unival_tree(TreeNode::from_vec(vec![1, 1, 1, 1, 1, null, 1])),
            true
        );
        assert_eq!(Solution::is_unival_tree(TreeNode::from_vec(vec![2, 2, 2, 5, 2])), false);
    }
}
