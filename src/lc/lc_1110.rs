// https://leetcode.com/problems/delete-nodes-and-return-forest/
// 1110. Delete Nodes And Return Forest
use std::cell::RefCell;
use std::rc::Rc;
// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}
pub struct Solution;
impl Solution {
    fn del_node_helper(
        root: Option<Rc<RefCell<TreeNode>>>,
        to_delete: &Vec<i32>,
        vec: &mut Vec<Option<Rc<RefCell<TreeNode>>>>,
        is_root: bool,
    ) -> bool {
        let mut delete_this = true;
        if let Some(node) = root {
            delete_this = to_delete.contains(&node.borrow().val);
            if is_root && !delete_this {
                vec.push(Some(node.clone()))
            }
            if Self::del_node_helper(node.borrow().left.clone(), to_delete, vec, delete_this) {
                node.borrow_mut().left = None;
            }
            if Self::del_node_helper(node.borrow().right.clone(), to_delete, vec, delete_this) {
                node.borrow_mut().right = None;
            }
        }
        delete_this
    }
    pub fn del_nodes(
        root: Option<Rc<RefCell<TreeNode>>>,
        to_delete: Vec<i32>,
    ) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        let mut vec = Vec::new();
        Self::del_node_helper(root, &to_delete, &mut vec, true);
        vec
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn from_array(vec: &Vec<i32>, i: usize) -> Option<Rc<RefCell<TreeNode>>> {
        if i >= vec.len() || vec[i] < 0 {
            return None;
        }
        let mut node = TreeNode::new(vec[i]);
        node.left = from_array(vec, i * 2 + 1);
        node.right = from_array(vec, i * 2 + 2);
        Some(Rc::new(RefCell::new(node)))
    }
    #[test]
    fn del_nodes() {
        assert_eq!(
            Solution::del_nodes(from_array(&vec![1, 2, 3, 4, 5, 6, 7], 0), vec![3, 5]),
            vec![
                from_array(&vec![1, 2, -1, 4], 0),
                from_array(&vec![6], 0),
                from_array(&vec![7], 0)
            ]
        );
        assert_eq!(
            Solution::del_nodes(from_array(&vec![1, 2, 4, -1, 3], 0), vec![3]),
            vec![from_array(&vec![1, 2, 4], 0)]
        );
    }
}
