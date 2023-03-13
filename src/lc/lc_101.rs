// https://leetcode.com/problems/symmetric-tree/
// 101. Symmetric Tree

pub struct Solution;
use super::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    fn is_symmetric_node(
        left: &Option<Rc<RefCell<TreeNode>>>,
        right: &Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        if let Some(left_node) = left {
            if let Some(right_node) = right {
                if left_node.borrow().val == right_node.borrow().val {
                    Self::is_symmetric_node(&left_node.borrow().right, &right_node.borrow().left)
                        && Self::is_symmetric_node(
                            &left_node.borrow().left,
                            &right_node.borrow().right,
                        )
                } else {
                    false
                }
            } else {
                false
            }
        } else if let None = right {
            true
        } else {
            false
        }
    }
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if let Some(node) = root {
            Self::is_symmetric_node(&node.borrow().left, &node.borrow().right)
        } else {
            true
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn is_symmetric() {
        assert_eq!(
            Solution::is_symmetric(TreeNode::from_vec(&vec![1, 2, 2, 3, 4, 4, 3])),
            true
        );
        assert_eq!(
            Solution::is_symmetric(TreeNode::from_vec(&vec![1, 2, 2, -1, 3, -1, 3])),
            false
        );
    }
}
