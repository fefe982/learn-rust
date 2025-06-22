// https://leetcode.com/problems/insert-into-a-binary-search-tree/
// 701. Insert into a Binary Search Tree
pub struct Solution;
use super::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn insert_into_bst(root: Option<Rc<RefCell<TreeNode>>>, val: i32) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(node) = root {
            if val < node.borrow().val {
                let n = Solution::insert_into_bst(node.borrow_mut().left.take(), val);
                node.borrow_mut().left = n;
            } else {
                let n = Solution::insert_into_bst(node.borrow_mut().right.take(), val);
                node.borrow_mut().right = n;
            }
            Some(node)
        } else {
            Some(Rc::new(RefCell::new(TreeNode::new(val))))
        }
    }
}
#[cfg(test)]
mod tests {
    use super::super::binary_tree::NULL;
    use super::*;
    #[test]
    fn insert_into_bst() {
        let null = NULL;
        assert_eq!(
            Solution::insert_into_bst(TreeNode::from_vec(vec![4, 2, 7, 1, 3]), 5),
            TreeNode::from_vec(vec![4, 2, 7, 1, 3, 5])
        );
        assert_eq!(
            Solution::insert_into_bst(TreeNode::from_vec(vec![40, 20, 60, 10, 30, 50, 70]), 25),
            TreeNode::from_vec(vec![40, 20, 60, 10, 30, 50, 70, null, null, 25])
        );
    }
}
