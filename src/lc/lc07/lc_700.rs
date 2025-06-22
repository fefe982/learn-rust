// https://leetcode.com/problems/search-in-a-binary-search-tree/
// 700. Search in a Binary Search Tree
pub struct Solution;
use super::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn search_bst(root: Option<Rc<RefCell<TreeNode>>>, val: i32) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(node) = root {
            let v = node.borrow().val;
            match v.cmp(&val) {
                std::cmp::Ordering::Equal => Some(node),
                std::cmp::Ordering::Less => Self::search_bst(node.borrow_mut().right.take(), val),
                std::cmp::Ordering::Greater => Self::search_bst(node.borrow_mut().left.take(), val),
            }
        } else {
            None
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn search_bst() {
        assert_eq!(
            Solution::search_bst(TreeNode::from_vec(vec![4, 2, 7, 1, 3]), 2),
            TreeNode::from_vec(vec![2, 1, 3])
        );
        assert_eq!(
            Solution::search_bst(TreeNode::from_vec(vec![4, 2, 7, 1, 3]), 5),
            TreeNode::from_vec(vec![])
        );
    }
}
