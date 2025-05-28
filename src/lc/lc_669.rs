// https://leetcode.com/problems/trim-a-binary-search-tree/
// 669. Trim a Binary Search Tree
pub struct Solution;
use super::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn trim_bst(root: Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(node) = root {
            let v = node.borrow().val;
            if v < low {
                return Self::trim_bst(node.borrow_mut().right.take(), low, high);
            } else if v > high {
                return Self::trim_bst(node.borrow_mut().left.take(), low, high);
            } else {
                {
                    let mut mnode = node.borrow_mut();
                    mnode.left = Self::trim_bst(mnode.left.take(), low, high);
                    mnode.right = Self::trim_bst(mnode.right.take(), low, high);
                }
                return Some(node);
            }
        } else {
            return None;
        }
    }
}
#[cfg(test)]
mod tests {
    use super::super::binary_tree::NULL;
    use super::*;
    #[test]
    fn trim_bst() {
        let null = NULL;
        assert_eq!(
            Solution::trim_bst(TreeNode::from_vec(vec![1, 0, 2]), 1, 2),
            TreeNode::from_vec(vec![1, null, 2])
        );
        assert_eq!(
            Solution::trim_bst(TreeNode::from_vec(vec![3, 0, 4, null, 2, null, null, 1]), 1, 3),
            TreeNode::from_vec(vec![3, 2, null, 1])
        );
    }
}
