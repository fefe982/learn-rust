// https://leetcode.com/problems/add-one-row-to-tree/
// 623. Add One Row to Tree
pub struct Solution;
use super::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    fn add_row(root: Option<Rc<RefCell<TreeNode>>>, val: i32, depth: i32, left: bool) -> Option<Rc<RefCell<TreeNode>>> {
        if depth == 1 {
            let mut n = TreeNode::new(val);
            if left {
                n.left = root;
            } else {
                n.right = root;
            }
            return Some(Rc::new(RefCell::new(n)));
        }
        if let Some(node) = root {
            let lnode = node.borrow_mut().left.take();
            let rnode = node.borrow_mut().right.take();
            node.borrow_mut().left = Self::add_row(lnode, val, depth - 1, true);
            node.borrow_mut().right = Self::add_row(rnode, val, depth - 1, false);
            return Some(node);
        }
        None
    }
    pub fn add_one_row(root: Option<Rc<RefCell<TreeNode>>>, val: i32, depth: i32) -> Option<Rc<RefCell<TreeNode>>> {
        Self::add_row(root, val, depth, true)
    }
}
#[cfg(test)]
mod tests {
    use super::super::binary_tree::NULL;
    use super::*;
    #[test]
    fn test_add_one_row() {
        let null = NULL;
        assert_eq!(
            Solution::add_one_row(TreeNode::from_vec(vec![4, 2, 6, 3, 1, 5]), 1, 2),
            TreeNode::from_vec(vec![4, 1, 1, 2, null, null, 6, 3, 1, 5])
        );
        assert_eq!(
            Solution::add_one_row(TreeNode::from_vec(vec![4, 2, null, 3, 1]), 1, 3),
            TreeNode::from_vec(vec![4, 2, null, 1, 1, 3, null, null, 1])
        );
    }
}
